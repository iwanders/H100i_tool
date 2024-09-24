use thiserror::Error;

/*
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\")" -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment

usb.dst == "1.3.0" or usb.src == "1.3.0"


preamble; 1c00204adf0482daffff000000001b000001000300000248000000002109000200004000
                                                            | random data, forgot init?                                      | checksum?
3f101400ff05ffffffffff00000000000000000000000000ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff59
pump quiet
3f101400ff05ffffffffff00000000000000000000000000ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff59
pump balanced
3fd81400ff05ffffffffff00000000000000000000000001ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff4d
pump extreme
3f581400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff43
coolant at 35.70 ish

fan1 balanced
3f581400ff05ffffffffff00000000000000000000000002ffff0000ff071c331e4f1f69208722ad23d424ff1e33204f2169238725ad29d42affffffffffff3a

fan1 extreme
3fa81400ff05ffffffffff00000000000000000000000002ffff0000ff071a591b6e1c871da31ebd1fdb20ff1e33204f2169238725ad29d42afffffffffffff2

fan1 quiet
3fc81400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff40

fan2 balanced
3f301400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1c331e4f1f69208722ad23d424fffffffffffff8

fan2 extreme
3fa01400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1a591b6e1c871da31ebd1fdb20ffffffffffffcf

fan2 quiet
3f081400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42afffffffffffff7

Requests come back from another endpoint?
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.1\") or (usb.src==\"1.3.1\")" -T fields -e frame.time -e usb.src -e usb.dst -e usb.capdata
ff:a0:12:08:00:17:13:92:24:00:00:a5:e8:03:a5:de:04:00:a5:e8:03:a5:ee:04:02:ff:00:00:ff:7f:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:81:8b:66:00:05:2d:32:37:41:80:24:00:00:00:00:00:00:00:44
ff:a8:12:08:00:18:13:92:24:00:00:a5:e8:03:a5:de:04:00:a5:e8:03:a5:ee:04:02:ff:00:00:ff:7f:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:a7:8b:66:00:05:2d:32:37:41:80:24:00:00:00:00:00:00:00:b2

tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst == \"1.3.1\") or (usb.src == \"1.3.1\") " -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment -e usb.capdata


in one:
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst == \"1.3.1\") or (usb.src == \"1.3.1\") " -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment



Sep 21, 2024 19:42:56.154897000 EDT	host	1.3.0	3f:c0:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:35
Sep 21, 2024 19:42:56.155178000 EDT	1.3.0	host
Sep 21, 2024 19:42:56.159726000 EDT	1.3.1	host		ff:88:12:08:00:9f:15:7d:23:00:00:90:e8:03:90:40:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:8b:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:40:e3:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:d3
Sep 21, 2024 19:42:56.159755000 EDT	host	1.3.1
Sep 21, 2024 19:42:57.669453000 EDT	host	1.3.0	3f:c8:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:da
Sep 21, 2024 19:42:57.669702000 EDT	1.3.0	host
Sep 21, 2024 19:42:57.675657000 EDT	1.3.1	host		ff:90:12:08:00:a0:15:7a:23:00:00:90:e8:03:90:41:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:81:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:36:e9:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:d4
Sep 21, 2024 19:42:57.675664000 EDT	host	1.3.1
Sep 21, 2024 19:42:59.199969000 EDT	host	1.3.0	3f:d0:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:ec
Sep 21, 2024 19:42:59.200252000 EDT	1.3.0	host
Sep 21, 2024 19:42:59.203696000 EDT	1.3.1	host		ff:98:12:08:00:a1:15:77:23:00:00:90:e8:03:90:41:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:79:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:3c:ef:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:81

second byte is sequence number, but it always increments by 8?

Reply is in URB_INTERRUPT

*/

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
