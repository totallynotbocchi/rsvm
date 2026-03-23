use strum::EnumCount as _;
use strum_macros::EnumCount;

#[derive(Debug, EnumCount)]
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
}
