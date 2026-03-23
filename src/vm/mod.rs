use crate::vm::{register::Register, stack::Stack};
use strum::EnumCount;

pub mod error;
pub mod instruction;
pub mod register;
pub mod stack;

pub struct VM {
    pub registers: [u32; Register::COUNT],
    pub stack: Stack,
}

impl VM {
    const DEFAULT_STACK_SIZE: usize = 1024;

    pub fn new(stack_capacity: usize) -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::new(stack_capacity),
        }
    }

    pub fn get_register(&self, register_id: Register) -> u32 {
        self.registers[register_id as usize]
    }

    pub fn set_register(&mut self, register_id: Register, value: u32) {
        self.registers[register_id as usize] = value;
    }
}

impl Default for VM {
    fn default() -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::new(Self::DEFAULT_STACK_SIZE),
        }
    }
}
