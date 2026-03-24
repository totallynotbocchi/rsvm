use strum::EnumCount as _;
use strum_macros::EnumCount;

use crate::vm::error::Error;

#[derive(Debug, EnumCount, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum Register {
    // general purpose
    G0,
    G1,
    G2,
    G3,

    // floating point
    F0,
    F1,
    F2,
    F3,

    // special
    SP,
    PC,
    RRG,
    FLGS,
}

// for handling registers referenced in bytes
impl TryFrom<usize> for Register {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::G0),
            1 => Ok(Self::G1),
            2 => Ok(Self::G2),
            3 => Ok(Self::G3),
            4 => Ok(Self::F0),
            5 => Ok(Self::F1),
            6 => Ok(Self::F2),
            7 => Ok(Self::F3),
            8 => Ok(Self::SP),
            9 => Ok(Self::PC),
            10 => Ok(Self::RRG),
            11 => Ok(Self::FLGS),
            _ => Err(Error::InvalidRegister),
        }
    }
}
