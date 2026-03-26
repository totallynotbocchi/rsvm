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
    is_running: bool,
}

impl CPU {
    // all FLGS register flags
    const FLAG_ZERO: u32 = (1 << 1); // if result is zero
    const FLAG_CARRY: u32 = (1 << 2); // if an unsigned operation overflows in the available bits
    const FLAG_NEGATIVE: u32 = (1 << 3); // sign of the result, 1 for negative
    const FLAG_OVERFLOW: u32 = (1 << 4); // if a signed operation weirdly changes sign
    const FLAG_ZERODIV: u32 = (1 << 5); // if a division by zero happened

    // methods

    pub fn new(ram_capacity: usize, stack_capacity: usize) -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::new(stack_capacity),
            ram: RAM::new(ram_capacity),
            pos: 0,
            is_running: false,
        }
    }

    // registers

    pub fn get_register(&self, register_id: &Register) -> u32 {
        self.registers[*register_id as usize]
    }

    pub fn set_register(&mut self, register_id: &Register, value: u32) {
        self.registers[*register_id as usize] = value;
    }

    pub fn add_flag(&mut self, flag: u32) {
        self.registers[Register::FLGS as usize] |= flag;
    }

    pub fn remove_flag(&mut self, flag: u32) {
        self.registers[Register::FLGS as usize] &= !flag;
    }

    pub fn is_flag_on(&mut self, flag: u32) -> bool {
        self.registers[Register::FLGS as usize] & flag == 1
    }

    // operations n shit

    fn resolve_binary_op(
        &mut self,
        op: &Opcode,
        a: &Register,
        b: &Register,
        c: &Register,
    ) -> Result<(), Error> {
        let a_value = self.get_register(a);
        let b_value = self.get_register(b);

        // check for division by zero
        // if there is then dont add anything in dest
        if (*op == Opcode::Div || *op == Opcode::Mod) && b_value == 0 {
            self.add_flag(Self::FLAG_ZERODIV);
            return Ok(());
        }

        // coompute the result
        let result: u32;
        match op {
            Opcode::Add => result = a_value.wrapping_add(b_value),
            Opcode::Sub => result = a_value.wrapping_sub(b_value),
            Opcode::Mul => result = a_value.wrapping_mul(b_value),
            Opcode::Div => result = a_value.wrapping_div(b_value),
            Opcode::Mod => result = a_value % b_value,
            _ => return Err(Error::InvalidInstruction),
        };

        // add flags
        // TODO: more of them
        if result == 0 {
            self.add_flag(Self::FLAG_ZERO);
        }

        // update the registers
        self.set_register(c, result);
        return Ok(());
    }

    // main shi

    pub fn main_loop(&mut self) -> Result<(), Error> {
        // prepare state
        self.pos = 0;
        self.is_running = true;

        // run loop
        while self.is_running {
            // get the current instruction
            let inst = self.ram.get_at(self.pos)?;
            self.execute(Instruction::decoded(inst)?)?;

            // advance
            self.pos += 1;
        }

        println!("Exiting...");
        Ok(())
    }

    pub fn execute(&mut self, inst: Instruction) -> Result<(), Error> {
        // for easier referencing in the future
        let oper1 = inst.get_source1();
        let oper2 = inst.get_source2();
        let dest = inst.get_dest();
        let opcode = inst.get_opcode();

        // label for supposedly cleaner branches
        match *opcode {
            // register case
            Opcode::PrintReg => {
                // print reg must have a register source
                if let Operand::Register(reg) = oper1 {
                    println!("Register {:?}: '{}'", reg, self.get_register(reg));
                    return Ok(());
                }
            }

            // nothing
            Opcode::Exit => {
                self.is_running = false;
                return Ok(());
            }

            // intermediate-register case
            Opcode::Put => {
                if *oper1 == Operand::Intermediate
                    && let Operand::Register(out_reg) = *dest
                {
                    self.pos += 1; // move to the next address to grab intermediate
                    let itm = self.ram.get_at(self.pos)?;

                    self.set_register(&out_reg, itm);
                    return Ok(());
                }
            }

            // register-register case
            Opcode::Mov => {
                if let Operand::Register(reg1) = *oper1
                    && let Operand::Register(out) = *dest
                {
                    let val = self.get_register(&reg1);
                    self.set_register(&out, val);
                    return Ok(());
                }
            }

            // binary operation case
            Opcode::Div | Opcode::Mul | Opcode::Add | Opcode::Sub | Opcode::Mod => {
                if let Operand::Register(a) = *oper1
                    && let Operand::Register(b) = *oper2
                    && let Operand::Register(c) = *dest
                {
                    self.resolve_binary_op(opcode, &a, &b, &c)?;
                    return Ok(());
                }
            }
        };

        return Err(Error::InvalidInstruction);
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: [0; Register::COUNT],
            stack: Stack::default(),
            ram: RAM::default(),
            pos: 0,
            is_running: false,
        }
    }
}
