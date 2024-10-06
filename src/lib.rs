use thiserror::Error;
use zerocopy::{AsBytes, FromBytes, FromZeroes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Error, Debug)]
pub enum H100iError {
    #[error("no matching usb device found")]
    NoDevice,
    #[error("hid error occurred")]
    HidError(#[from] hidapi::HidError),
    #[error("parse error occurred")]
    ParseError((String, [u8; 64])),
    #[error("crc error occurred")]
    CrcError([u8; 64]),
}

mod wire;

#[derive(Copy, Clone, Debug, Default)]
pub struct DutyCycle(pub u8);

#[derive(Copy, Clone, Debug, Default)]
pub struct Rpm(pub u16);

#[derive(Copy, Clone, Debug, Default)]
pub struct TemperatureC(pub f32);

#[derive(Copy, Clone, Debug, Default)]
pub struct FanStatus {
    pub speed: Rpm,
    pub duty_cycle: DutyCycle,
}

#[derive(Copy, Clone, Debug)]
pub struct StatusMsg {
    pub temperature_1: TemperatureC,
    pub temperature_2: TemperatureC,
    pub uptime_ms: u32,
    pub msg_counter: u16,
    pub fans: [FanStatus; 4],
}

#[derive(Copy, Clone, Debug)]
pub enum Msg {
    Status(StatusMsg),
}

#[derive(Debug)]
pub struct H100i {
    // api: hidapi::HidApi,
    device: hidapi::HidDevice,
    sequence: u8,
}

impl H100i {
    pub fn new() -> Result<H100i, H100iError> {
        // Bus 001 Device 003: ID 1b1c:0c35 Corsair
        let vendor_id = 0x1b1c;
        let product_id = 0x0c35;

        let api = hidapi::HidApi::new()?;
        let mut found_device = None;
        for device in api.device_list() {
            if device.vendor_id() == vendor_id && device.product_id() == product_id {
                found_device = Some(device.open_device(&api)?);
            }
        }
        if let Some(device) = found_device {
            device.set_blocking_mode(true)?;
            Ok(H100i {
                device,
                sequence: 0x90,
            })
        } else {
            Err(H100iError::NoDevice)
        }
    }

    fn advance_sequence(&mut self) -> u8 {
        let v = self.sequence.wrapping_add(8);
        self.sequence = v;
        v
    }
    /// Helper function to prepend a zero to a byte slice, send_feature_report requires this.
    fn prepend_zero(v: &[u8]) -> Vec<u8> {
        let mut new_v: Vec<u8> = Vec::new();
        new_v.push(0);
        for i in 0..v.len() {
            new_v.push(v[i])
        }
        return new_v;
    }

    pub fn get_status_bytes(&mut self) -> Result<[u8; 64], H100iError> {
        // 3f:c0:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:35
        let mut msg = wire::Msg::new();
        let new_sequence = self.advance_sequence();
        msg.sequence = new_sequence;
        msg.command = 0xff;
        msg.update_crc();

        self.device.write(&Self::prepend_zero(msg.as_bytes()))?;

        // And collect the answer.
        let mut resp = [0u8; 64];
        self.device.read(&mut resp)?;

        Ok(resp)
    }

    pub fn get_status(&mut self) -> Result<wire::Status, H100iError> {
        let bytes = self.get_status_bytes()?;
        Ok(*wire::Status::ref_from(&bytes).unwrap())
    }
}

pub fn main() -> Result<(), H100iError> {
    let mut d = H100i::new()?;
    // println!("d: {d:?}");
    loop {
        let status = d.get_status()?;
        println!("Status: {status:#?}");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}
