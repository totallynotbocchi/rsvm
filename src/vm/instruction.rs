use core::fmt;

use crate::vm::{error::Error, register::Register};
use strum::EnumCount as _;
use strum_macros::EnumCount;

#[repr(u8)]
#[derive(Debug, Copy, Clone, EnumCount, PartialEq)]
pub enum Opcode {
    PrintReg, // src1: register name
    Put,      // src1: next intermediate, dest: reg to put in
    Mov,      // src1: reg to take from, dest: reg to move in
    Add,      // src1: reg to add, src2: second reg to add, dest: output of operation
    Sub,      // src1: reg to subtract from, src2: second reg to subtract, dest: output of operation
    Mul,      // src1: first value, src2: second value, dest: product
    Div,      // src1: divident, src2: divisor, dest: quotient
    Mod,      // src1: divident, src2: divisor, dest: remainder
}

impl TryFrom<u8> for Opcode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::PrintReg),
            1 => Ok(Self::Put),
            2 => Ok(Self::Mov),
            3 => Ok(Self::Add),
            4 => Ok(Self::Sub),
            5 => Ok(Self::Mul),
            6 => Ok(Self::Div),
            7 => Ok(Self::Mod),
            _ => Err(Error::InvalidOpcode),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
    None,
    Address,      // always the next value in memory
    Intermediate, // always the next value in memory
    Register(Register),
}

// this is ugly, yes, but its probably the cleanest solution
impl TryFrom<u8> for Operand {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Intermediate),
            2 => Ok(Self::Address),
            0xA0 => Ok(Self::Register(Register::G0)),
            0xA1 => Ok(Self::Register(Register::G1)),
            0xA2 => Ok(Self::Register(Register::G2)),
            0xA3 => Ok(Self::Register(Register::G3)),
            0xB0 => Ok(Self::Register(Register::F0)),
            0xB1 => Ok(Self::Register(Register::F1)),
            0xB2 => Ok(Self::Register(Register::F2)),
            0xB3 => Ok(Self::Register(Register::F3)),
            0xC0 => Ok(Self::Register(Register::SP)),
            0xC1 => Ok(Self::Register(Register::PC)),
            0xC2 => Ok(Self::Register(Register::RRG)),
            0xC3 => Ok(Self::Register(Register::FLGS)),
            _ => Err(Error::InvalidOperand),
        }
    }
}

impl TryInto<u32> for Operand {
    type Error = Error;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Self::None => Ok(0),
            Self::Intermediate => Ok(1),
            Self::Address => Ok(2),
            Self::Register(Register::G0) => Ok(0xA0),
            Self::Register(Register::G1) => Ok(0xA1),
            Self::Register(Register::G2) => Ok(0xA2),
            Self::Register(Register::G3) => Ok(0xA3),
            Self::Register(Register::F0) => Ok(0xB0),
            Self::Register(Register::F1) => Ok(0xB1),
            Self::Register(Register::F2) => Ok(0xB2),
            Self::Register(Register::F3) => Ok(0xB3),
            Self::Register(Register::SP) => Ok(0xC0),
            Self::Register(Register::PC) => Ok(0xC1),
            Self::Register(Register::RRG) => Ok(0xC2),
            Self::Register(Register::FLGS) => Ok(0xC3),
        }
    }
}

// each instruction is 32 bit, split into:
// [ OP (8 bytes), SRC1 (8 bytes), SRC2(8 bytes), DEST (8 bytes) ]
pub struct Instruction {
    opcode: Opcode,
    src1: Operand,
    src2: Operand,
    dest: Operand,
}

impl Instruction {
    pub fn new(opcode: Opcode, src1: Operand, src2: Operand, dest: Operand) -> Self {
        Self {
            opcode,
            src1,
            src2,
            dest,
        }
    }

    // TODO: turn this into TryFrom<u32> maybe?
    pub fn decoded(packed_inst: u32) -> Result<Self, Error> {
        let opcode = Opcode::try_from(((packed_inst >> 24) & 0xFF) as u8)?;
        let src1 = Operand::try_from(((packed_inst >> 16) & 0xFF) as u8)?;
        let src2 = Operand::try_from(((packed_inst >> 8) & 0xFF) as u8)?;
        let dest = Operand::try_from(((packed_inst >> 0) & 0xFF) as u8)?;

        Ok(Self {
            opcode,
            src1,
            src2,
            dest,
        })
    }

    pub fn packed(&self) -> Result<u32, Error> {
        // this is for easier reading
        let opcode = self.opcode as u32;
        let src1: u32 = self.src1.try_into()?;
        let src2: u32 = self.src2.try_into()?;
        let dest: u32 = self.dest.try_into()?;

        // return packed as an u32 instruction
        Ok((opcode << 24) | (src1 << 16) | (src2 << 8) | (dest << 0))
    }

    pub fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    pub fn get_source1(&self) -> &Operand {
        &self.src1
    }

    pub fn get_source2(&self) -> &Operand {
        &self.src2
    }

    pub fn get_dest(&self) -> &Operand {
        &self.dest
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "INSTRUCTION: {{\n\tOPCODE: {:?},\n\tSRC1: {:?},\n\tSRC2: {:?},\n\tDEST: {:?}\n}}",
            self.opcode, self.src1, self.src2, self.dest
        )
    }
}
