use strum::EnumCount;

use crate::vm::{
    error::Error,
    instruction::{Instruction, Opcode, Operand},
    memory::{RAM, Stack},
    register::Register,
};

pub struct CPU {
    pub registers: [u32; Register::COUNT],
    pub stack: Stack,
    pub ram: RAM,
    pos: usize,
}

impl CPU {
    pub fn new(ram_capacity: usize, stack_capacity: usize) -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::new(stack_capacity),
            ram: RAM::new(ram_capacity),
            pos: 0,
        }
    }

    pub fn get_register(&self, register_id: &Register) -> u32 {
        self.registers[*register_id as usize]
    }

    pub fn set_register(&mut self, register_id: &Register, value: u32) {
        self.registers[*register_id as usize] = value;
    }

    pub fn execute(&mut self, pos: usize, inst: Instruction) -> Result<(), Error> {
        let oper1 = inst.get_source1();
        let oper2 = inst.get_source2();
        let dest = inst.get_dest();

        // label for supposedly cleaner branches
        'parsing: {
            match *inst.get_opcode() {
                Opcode::PrintReg => {
                    // print reg must have a register source
                    if let Operand::Register(reg) = oper1 {
                        println!("Register {:?}: '{}'", reg, self.get_register(reg));
                        break 'parsing;
                    }

                    return Err(Error::InvalidInstruction);
                }

                Opcode::Put => {
                    if *oper1 == Operand::Intermediate
                        && let Operand::Register(out_reg) = *dest
                    {
                        let itm = self.ram.get_at(pos + 1)?;
                        self.set_register(&out_reg, itm);

                        break 'parsing;
                    }

                    return Err(Error::InvalidInstruction);
                }

                Opcode::Mov => {
                    if let Operand::Register(reg1) = *oper1
                        && let Operand::Register(out) = *dest
                    {
                        let val = self.get_register(&reg1);
                        self.set_register(&out, val);
                        break 'parsing;
                    }

                    return Err(Error::InvalidInstruction);
                }
            };
        }

        // always ok cus errors are checked in the branches
        Ok(())
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::default(),
            ram: RAM::default(),
            pos: 0,
        }
    }
}
