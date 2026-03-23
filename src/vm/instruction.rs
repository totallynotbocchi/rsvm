use core::fmt;

use crate::vm::error::Error;
use strum::EnumCount as _;
use strum_macros::EnumCount;

#[repr(u8)]
#[derive(Debug, Copy, Clone, EnumCount)]
pub enum Opcode {
    PrintReg, // src1: register name
}

impl TryFrom<u8> for Opcode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::PrintReg),
            _ => Err(Error::InvalidOpcode),
        }
    }
}

// each instruction is 32 bit, split into:
// [ OP (8 bytes), SRC1 (8 bytes), SRC2(8 bytes), DEST (8 bytes) ]
pub struct Instruction {
    opcode: Opcode,
    src1: u8,
    src2: u8,
    dest: u8,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self {
            opcode,
            src1: 0,
            src2: 0,
            dest: 0,
        }
    }

    pub fn decoded(packed_inst: u32) -> Result<Self, Error> {
        let opcode = Opcode::try_from(((packed_inst >> 24) & 0xFF) as u8)?;
        let src1 = ((packed_inst >> 16) & 0xFF) as u8;
        let src2 = ((packed_inst >> 8) & 0xFF) as u8;
        let dest = ((packed_inst >> 0) & 0xFF) as u8;

        Ok(Self {
            opcode,
            src1,
            src2,
            dest,
        })
    }

    pub fn packed(&self) -> u32 {
        // this is for easier reading
        let opcode = self.opcode as u32;
        let src1 = self.src1 as u32;
        let src2 = self.src2 as u32;
        let dest = self.dest as u32;

        // return packed as an u32 instruction
        (opcode << 24) | (src1 << 16) | (src2 << 8) | (dest << 0)
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "INSTRUCTION: {{\n\tOPCODE: {:?},\n\tSRC1: {},\n\tSRC2: {},\n\tDEST: {}\n}}",
            self.opcode, self.src1, self.src2, self.dest
        )
    }
}
