use zerocopy::{AsBytes, FromBytes, FromZeroes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

// Must be 64 bytes long, last byte is crc.
#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct Msg {
    /// Magic byte, always 0x3f.
    pub magic: u8,
    /// Sequence increments by 8s
    pub sequence: u8,
    /// Command byte
    pub command: u8,
    // big unknown here
    pub padding: [u8; 60],
    /// Crc value, calculated from all but the magic byte.
    pub crc: u8,
}

const MSG_SIZE: usize = 64;
const _: () = assert!(
    std::mem::size_of::<Msg>() == MSG_SIZE,
    "msg is known to be 64 bytes"
);
impl Msg {
    pub fn new() -> Self {
        let mut msg = Self::new_zeroed();
        msg.magic = 0x3f;
        return msg;
    }
    pub fn update_crc(&mut self) {
        let data = self.as_bytes();
        let value = crc8(&data[1..MSG_SIZE - 1]);
        self.crc = value;
    }
}

fn crc8(data: &[u8]) -> u8 {
    // Table from:
    // c = crcmod.predefined.mkCrcFun("crc-8")
    // >>> with open("crc8.cpp", "w") as f:
    // ...    c.generateCode("crc8", f)
    const TABLE: [u8; 256] = [
        0x00, 0x07, 0x0E, 0x09, 0x1C, 0x1B, 0x12, 0x15, 0x38, 0x3F, 0x36, 0x31, 0x24, 0x23, 0x2A,
        0x2D, 0x70, 0x77, 0x7E, 0x79, 0x6C, 0x6B, 0x62, 0x65, 0x48, 0x4F, 0x46, 0x41, 0x54, 0x53,
        0x5A, 0x5D, 0xE0, 0xE7, 0xEE, 0xE9, 0xFC, 0xFB, 0xF2, 0xF5, 0xD8, 0xDF, 0xD6, 0xD1, 0xC4,
        0xC3, 0xCA, 0xCD, 0x90, 0x97, 0x9E, 0x99, 0x8C, 0x8B, 0x82, 0x85, 0xA8, 0xAF, 0xA6, 0xA1,
        0xB4, 0xB3, 0xBA, 0xBD, 0xC7, 0xC0, 0xC9, 0xCE, 0xDB, 0xDC, 0xD5, 0xD2, 0xFF, 0xF8, 0xF1,
        0xF6, 0xE3, 0xE4, 0xED, 0xEA, 0xB7, 0xB0, 0xB9, 0xBE, 0xAB, 0xAC, 0xA5, 0xA2, 0x8F, 0x88,
        0x81, 0x86, 0x93, 0x94, 0x9D, 0x9A, 0x27, 0x20, 0x29, 0x2E, 0x3B, 0x3C, 0x35, 0x32, 0x1F,
        0x18, 0x11, 0x16, 0x03, 0x04, 0x0D, 0x0A, 0x57, 0x50, 0x59, 0x5E, 0x4B, 0x4C, 0x45, 0x42,
        0x6F, 0x68, 0x61, 0x66, 0x73, 0x74, 0x7D, 0x7A, 0x89, 0x8E, 0x87, 0x80, 0x95, 0x92, 0x9B,
        0x9C, 0xB1, 0xB6, 0xBF, 0xB8, 0xAD, 0xAA, 0xA3, 0xA4, 0xF9, 0xFE, 0xF7, 0xF0, 0xE5, 0xE2,
        0xEB, 0xEC, 0xC1, 0xC6, 0xCF, 0xC8, 0xDD, 0xDA, 0xD3, 0xD4, 0x69, 0x6E, 0x67, 0x60, 0x75,
        0x72, 0x7B, 0x7C, 0x51, 0x56, 0x5F, 0x58, 0x4D, 0x4A, 0x43, 0x44, 0x19, 0x1E, 0x17, 0x10,
        0x05, 0x02, 0x0B, 0x0C, 0x21, 0x26, 0x2F, 0x28, 0x3D, 0x3A, 0x33, 0x34, 0x4E, 0x49, 0x40,
        0x47, 0x52, 0x55, 0x5C, 0x5B, 0x76, 0x71, 0x78, 0x7F, 0x6A, 0x6D, 0x64, 0x63, 0x3E, 0x39,
        0x30, 0x37, 0x22, 0x25, 0x2C, 0x2B, 0x06, 0x01, 0x08, 0x0F, 0x1A, 0x1D, 0x14, 0x13, 0xAE,
        0xA9, 0xA0, 0xA7, 0xB2, 0xB5, 0xBC, 0xBB, 0x96, 0x91, 0x98, 0x9F, 0x8A, 0x8D, 0x84, 0x83,
        0xDE, 0xD9, 0xD0, 0xD7, 0xC2, 0xC5, 0xCC, 0xCB, 0xE6, 0xE1, 0xE8, 0xEF, 0xFA, 0xFD, 0xF4,
        0xF3,
    ];
    use std::num::Wrapping;
    let mut crc: Wrapping<u8> = Wrapping(0);
    for v in data.iter() {
        crc = Wrapping(TABLE[(v ^ crc.0) as usize]);
    }
    // while (len > 0)
    // {
    // crc = table[*data ^ (UINT8)crc];
    // data++;
    // len--;
    // }
    crc.0
}

#[derive(Copy, Clone, FromZeroes, FromBytes, AsBytes)]
#[repr(C, packed(1))]
pub struct FanStatus {
    pub duty_1: u8,
    pub _e8: u8,
    pub _is03: u8,
    pub duty_2: u8,

    pub value: u16, // This is unaligned, bah.

    pub _pad: u8,
}
const _: () = assert!(
    std::mem::size_of::<FanStatus>() == 7,
    "FanStatus is known to be 7 bytes"
);

impl std::fmt::Debug for FanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value; // copy because it's unaligned :(
        f.debug_struct("FanStatus")
            .field("value", &value)
            .field("duty_1", &self.duty_1)
            .field("duty_2", &self.duty_2)
            .finish()
    }
}

// Must be 64 bytes long, last byte is crc.
#[derive(Copy, Clone, FromZeroes, FromBytes, AsBytes)]
#[repr(C, packed(1))]
pub struct Status {
    pub cmd: u8,
    pub seq: u8,
    pub _always_12: u8,
    pub _always_08: u8,
    pub _pad: u8,
    pub msg_counter: u16,
    pub value_start_t1: u16,
    pub _pad2: u16, // always zeros

    pub fans: [FanStatus; 4],

    pub _something_le: u16,
    pub _something_be: u16,

    pub _pad3: u16,

    pub uptime_ms: u32,

    pub _some_id: [u8; 5], //0x052d323741

    pub value_end_t1: u16,

    pub _pad_5_zero: [u8; 7],
    pub crc: u8,
}
impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let value = self.value;  // copy because it's unaligned :(
        let value_start_t1 = self.value_start_t1;
        let value_end_t1 = self.value_end_t1;
        let msg_counter = self.msg_counter;
        let uptime_ms = self.uptime_ms;
        f.debug_struct("Status")
            .field("value_start_t1", &value_start_t1)
            .field("value_end_t1", &value_end_t1)
            .field("msg_counter", &msg_counter)
            .field("uptime_ms", &uptime_ms)
            .field("fans", &self.fans)
            // .field("duty_1", &self.duty_1)
            // .field("duty_2", &self.duty_2)
            .finish()
    }
}

const _: () = assert!(
    std::mem::size_of::<Status>() == MSG_SIZE,
    "msg is known to be 64 bytes"
);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_crc() {
        assert_eq!(crc8(&[0, 1, 2, 3]), 0x48);
        assert_eq!(crc8(&[30, 12, 24, 36]), 0x5a);
        assert_eq!(
            crc8(&[
                136, 18, 8, 0, 159, 21, 125, 35, 0, 0, 144, 232, 3, 144, 64, 4, 0, 144, 232, 3,
                144, 82, 4, 2, 255, 0, 0, 255, 139, 11, 0, 0, 0, 0, 0, 0, 0, 0, 69, 50, 51, 69, 0,
                0, 64, 227, 111, 0, 5, 45, 50, 55, 65, 106, 35, 0, 0, 0, 0, 0, 0, 0
            ]),
            0xd3
        );
    }

    #[test]
    fn test_status() {
        let on_wire = [
            255, 224, 18, 8, 0, 223, 18, 199, 34, 0, 0, 132, 232, 3, 132, 216, 3, 0, 132, 232, 3,
            132, 235, 3, 2, 255, 0, 0, 255, 128, 11, 0, 0, 0, 0, 0, 0, 0, 0, 35, 1, 1, 35, 0, 0,
            45, 188, 186, 0, 5, 45, 50, 55, 65, 221, 34, 0, 0, 0, 0, 0, 0, 0, 218u8,
        ];
        assert_eq!(on_wire.len(), MSG_SIZE);
        let status = Status::ref_from(&on_wire);
        println!("status: {status:#?}");
        println!("status size: {}", std::mem::size_of::<Status>());
    }
}
