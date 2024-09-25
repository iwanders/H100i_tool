use thiserror::Error;

#[derive(Error, Debug)]
pub enum H100iError {
    #[error("no matching usb device found")]
    NoDevice,
    #[error("hid error occured")]
    HidError(#[from] hidapi::HidError),
}

#[derive(Debug)]
pub struct H100i {
    // api: hidapi::HidApi,
    device: hidapi::HidDevice,
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
            Ok(H100i { device })
        } else {
            Err(H100iError::NoDevice)
        }
    }
}

pub fn main() -> Result<(), H100iError> {
    let d = H100i::new()?;
    println!("d: {d:?}");
    Ok(())
}
