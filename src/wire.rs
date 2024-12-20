use crate::H100iError;
use zerocopy::{AsBytes, FromBytes, FromZeroes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

// Must be 64 bytes long, last byte is crc.
#[derive(FromZeroes, FromBytes, AsBytes, Copy, Clone)]
#[repr(C)]
pub struct Msg {
    /// Magic byte, always 0x3f.
    pub magic: u8,
    /// Sequence increments by 8s
    pub sequence: u8,
    /// Command byte
    pub command: u8,
    // big unknown here
    pub payload: [u8; 60],
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
    pub fn from_slice(bytes: &[u8]) -> Result<Self, H100iError> {
        Self::ref_from(bytes)
            .copied()
            .ok_or(H100iError::ParseLengthError)
    }

    pub fn update_crc(&mut self) {
        let data = self.as_bytes();
        let value = crc8(&data[1..MSG_SIZE - 1]);
        self.crc = value;
    }

    pub fn is_valid(&self) -> bool {
        crc8(&self.as_bytes()[1..MSG_SIZE - 1]) == self.crc
    }

    fn as_array(&self) -> [u8; 64] {
        let mut v = [0u8; 64];
        v.copy_from_slice(self.as_bytes());
        v
    }

    pub fn parse(&self) -> Result<crate::Msg, H100iError> {
        if !self.is_valid() {
            let mut res = [0u8; 64];
            res.copy_from_slice(self.as_bytes());
            return Err(H100iError::CrcError(res));
        }
        use crate::{DutyCycle, FanStatus, Msg, Rpm, StatusMsg, TemperatureC};

        if self.command == 0x12 {
            if self.magic != 0xff {
                return Err(H100iError::ParseError((
                    "magic was not 0xff".to_owned(),
                    self.as_array(),
                )));
            }
            let wire_status = Status::ref_from(&self.payload).ok_or(H100iError::ParseError((
                "couldn't parse payload".to_owned(),
                self.as_array(),
            )))?;
            let mut fans: [FanStatus; 4] = Default::default();
            for (i, fan) in wire_status.fans.iter().enumerate() {
                fans[i] = crate::FanStatus {
                    duty_cycle: DutyCycle(fan.duty_1),
                    speed: Rpm(fan.value),
                };
            }

            return Ok(Msg::Status(StatusMsg {
                msg_counter: wire_status.msg_counter,
                uptime_ms: wire_status.uptime_ms,
                temperature_1: TemperatureC(wire_status.value_start_t1.as_f32()),
                temperature_2: TemperatureC(wire_status.value_end_t1.as_f32()),
                // msg_counter: wire_status.msg_counter;
                fans,
            }));
        }
        todo!("need to flesh out the nice messages")
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
pub struct TempStatus {
    pub frac: u8,
    pub deg: u8,
}
impl std::fmt::Debug for TempStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.as_f32();
        f.debug_struct("TempStatus ")
            .field("v_C", &value)
            // .field("deg", &self.deg)
            // .field("frac", &self.frac)
            .finish()
    }
}
impl TempStatus {
    pub fn as_f32(&self) -> f32 {
        self.deg as f32 + self.frac as f32 / 255.0
    }
}

#[derive(Copy, Clone, FromZeroes, FromBytes, AsBytes)]
#[repr(C, packed(1))]
pub struct FanStatus {
    // duty 1 and 2 are always identical?
    pub duty_1: u8,
    // only e8 for fan
    pub _e8: u8,
    // only 03 for fan
    pub _is03: u8,

    pub duty_2: u8,

    // This is unaligned, bah, does it need a divisor for fans, seems high?
    pub value: u16,

    // Is 2 for the second fan?
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
            // .field("_is03", &self._is03)
            // .field("_e8", &self._e8)
            .finish()
    }
}

// Must be 64 bytes long, last byte is crc.
#[derive(Copy, Clone, FromZeroes, FromBytes, AsBytes)]
#[repr(C, packed(1))]
pub struct Status {
    // pub cmd: u8,
    // pub seq: u8,
    // pub _always_12: u8,
    pub _always_08: u8,
    pub _pad: u8,
    pub msg_counter: u16,

    // t1 changes more often on cooldown than t2, is inflow, one outflow?
    pub value_start_t1: TempStatus,

    pub _pad2: u16, // always zeros

    pub fans: [FanStatus; 4],

    // Is this a temperature in Kelvin? Why in both endianness?
    pub _something_le: u16,
    pub _something_be: u16,

    pub _pad3: u16,
    // pretty sure about this, increments change exactly with used delay.
    pub uptime_ms: u32,

    pub _some_id: [u8; 5], //0x052d323741

    pub value_end_t1: TempStatus,

    pub _pad_5_zero: [u8; 7],
    // pub crc: u8,
}
impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let value = self.value;  // copy because it's unaligned :(
        let value_start_t1 = self.value_start_t1;
        let value_end_t1 = self.value_end_t1;
        let msg_counter = self.msg_counter;
        let uptime_ms = self.uptime_ms;
        let _something_le = self._something_le;
        f.debug_struct("Status")
            .field("value_start_t1", &value_start_t1)
            .field("value_end_t1", &value_end_t1)
            .field("msg_counter", &msg_counter)
            .field("uptime_ms", &uptime_ms)
            // .field("_something_le", &_something_le)
            .field("fans", &self.fans)
            .finish()
    }
}

const _: () = assert!(
    std::mem::size_of::<Status>() == MSG_SIZE - 4,
    "msg is known to be 64 bytes"
);

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromZeroes, FromBytes, AsBytes)]
#[repr(C, packed(1))]
pub struct CurvePoint {
    pub temperature: u8,
    pub duty: u8,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromZeroes, FromBytes, AsBytes)]
#[repr(C, packed(1))]
pub struct CoolingCurve {
    pub curve: [CurvePoint; 7],
}
// 1e33
// 204f
// 2169
// 2387
// 25ad
// 29d4
// 2aff

impl CoolingCurve {
    pub fn balanced() -> Self {
        CoolingCurve {
            curve: [
                CurvePoint {
                    temperature: 30,
                    duty: 51,
                },
                CurvePoint {
                    temperature: 32,
                    duty: 79,
                },
                CurvePoint {
                    temperature: 33,
                    duty: 105,
                },
                CurvePoint {
                    temperature: 35,
                    duty: 135,
                },
                CurvePoint {
                    temperature: 37,
                    duty: 173,
                },
                CurvePoint {
                    temperature: 41,
                    duty: 212,
                },
                CurvePoint {
                    temperature: 42,
                    duty: 255,
                },
            ],
        }
    }
    pub fn extreme() -> Self {
        CoolingCurve {
            curve: [
                CurvePoint {
                    temperature: 26,
                    duty: 89,
                },
                CurvePoint {
                    temperature: 27,
                    duty: 110,
                },
                CurvePoint {
                    temperature: 28,
                    duty: 135,
                },
                CurvePoint {
                    temperature: 29,
                    duty: 163,
                },
                CurvePoint {
                    temperature: 30,
                    duty: 189,
                },
                CurvePoint {
                    temperature: 31,
                    duty: 219,
                },
                CurvePoint {
                    temperature: 32,
                    duty: 255,
                },
            ],
        }
    }
    pub fn quiet() -> Self {
        CoolingCurve {
            curve: [
                CurvePoint {
                    temperature: 30,
                    duty: 51,
                },
                CurvePoint {
                    temperature: 32,
                    duty: 79,
                },
                CurvePoint {
                    temperature: 33,
                    duty: 105,
                },
                CurvePoint {
                    temperature: 35,
                    duty: 135,
                },
                CurvePoint {
                    temperature: 37,
                    duty: 173,
                },
                CurvePoint {
                    temperature: 41,
                    duty: 212,
                },
                CurvePoint {
                    temperature: 42,
                    duty: 255,
                },
            ],
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum PumpMode {
    Quiet,
    #[default]
    Balanced,
    Extreme,
}

/*
    For cooling two types are seen:

Type 1, speciying pump and curves
   3fd81400ff05ffffffffff00000000000000000000000001ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff4d

Type 2,
   3f681400ff05ffffffffffffffffffffffffffffffffff02ffffd422ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff36
                                                       ^^^^ Perhaps temperature setpoint? See protocol.md for speculation.
*/
#[derive(Copy, Clone, FromZeroes, FromBytes, AsBytes)]
#[repr(C, packed(1))]
pub struct SetCooling {
    // pub cmd: u8,
    // pub seq: u8,
    // pub always_14: u8,
    pub always_00_1: u8,
    pub always_ff_1: u8,
    pub always_05: u8,
    pub always_ff_2: [u8; 5],
    pub always_00_2: [u8; 12],
    pub pump: u8,
    pub always_ff_3: [u8; 2],

    pub type_2_varies: u8, // type 1 is zero
    pub type_2_22: u8,     // type 1 is zero

    pub always_ff_4: u8,
    pub type_1_07: u8, // type 2 is ff
    pub curves: [CoolingCurve; 2],

    pub always_ff_5: [u8; 5],
    // pub crc: u8,
}
const _: () = assert!(
    std::mem::size_of::<SetCooling>() == MSG_SIZE - 4,
    "msg is known to be 64 bytes"
);
impl std::fmt::Debug for SetCooling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetCooling")
            .field("pump", &self.pump)
            .field("curves", &self.curves)
            .finish()
    }
}

impl SetCooling {
    pub fn from_config(config: &crate::Config) -> Self {
        SetCooling {
            always_00_1: 0,
            always_ff_1: 0xff,
            always_05: 5,
            always_ff_2: [0xff; 5],
            always_00_2: [0x0; 12],
            pump: config.pump as u8,
            always_ff_3: [0xff; 2],
            type_2_varies: 0,
            type_2_22: 0,
            always_ff_4: 0xff,
            type_1_07: 7,
            curves: config.fans,
            always_ff_5: [0xff; 5],
        }
    }
}

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
        let status = Status::ref_from(&on_wire[3..MSG_SIZE - 1]);
        assert!(status.is_some());
        let status = status.unwrap();
        // assert!(status.is_valid());
        println!("status: {status:#?}");
        println!("status size: {}", std::mem::size_of::<Status>());

        let wire_msg = Msg::ref_from(&on_wire).expect("should be parsable");
        let parsed = wire_msg.parse();
        println!("parsed: {parsed:?}");
        if let Ok(crate::Msg::Status(status)) = parsed {
            println!("status: {status:#?}");
        } else {
            assert!(false, "was not status message");
        }
    }
    #[test]
    fn test_set_cooling() {
        let on_wire = [
            63, 216, 20, 0, 255, 5, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            255, 255, 0, 0, 255, 7, 30, 51, 32, 79, 33, 105, 35, 135, 37, 173, 41, 212, 42, 255,
            30, 51, 32, 79, 33, 105, 35, 135, 37, 173, 41, 212, 42, 255, 255, 255, 255, 255, 255,
            77u8,
        ];
        assert_eq!(on_wire.len(), MSG_SIZE);
        let set_cooling = SetCooling::ref_from(&on_wire[3..MSG_SIZE - 1]);
        assert!(set_cooling.is_some());
        let set_cooling = set_cooling.unwrap();
        // assert!(set_cooling.is_valid());
        println!("set_cooling: {set_cooling:#?}");
        for curve in set_cooling.curves.iter() {
            assert_eq!(curve, &CoolingCurve::balanced());
        }
    }
}
