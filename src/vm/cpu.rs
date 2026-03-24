use strum::EnumCount;

use crate::vm::{
    error::Error,
    instruction::{Instruction, Opcode},
    memory::{RAM, Stack},
    register::Register,
};

pub struct CPU {
    pub registers: [u32; Register::COUNT],
    pub stack: Stack,
    pub ram: RAM,
}

impl CPU {
    pub fn new(ram_capacity: usize, stack_capacity: usize) -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::new(stack_capacity),
            ram: RAM::new(ram_capacity),
        }
    }

    pub fn get_register(&self, register_id: &Register) -> u32 {
        self.registers[*register_id as usize]
    }

    pub fn set_register(&mut self, register_id: &Register, value: u32) {
        self.registers[*register_id as usize] = value;
    }

    pub fn execute(&self, inst: Instruction) -> Result<(), Error> {
        match *inst.get_opcode() {
            Opcode::PrintReg => {
                // transform number into register
                let reg = Register::try_from(*inst.get_source1() as usize)?;
                println!("Register {:?}: '{}'", reg, self.get_register(&reg));
            }
            _ => return Err(Error::InvalidOpcode),
        };

        Ok(())
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::default(),
            ram: RAM::default(),
        }
    }
}
