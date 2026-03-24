pub mod vm;

use vm::*;

use crate::vm::{
    cpu::CPU,
    instruction::{Instruction, Opcode, Operand},
};

fn main() {
    let mut cpu = CPU::default();

    match cpu.ram.set_at(1, 2) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {:?}", e),
    }

    let ins1 = Instruction::new(
        Opcode::Put,
        Operand::Intermediate,
        Operand::None,
        Operand::Register(register::Register::G0),
    );

    let ins2 = Instruction::new(
        Opcode::PrintReg,
        Operand::Register(register::Register::G0),
        Operand::None,
        Operand::None,
    );

    match cpu.execute(0, ins1) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {:?}", e),
    }

    match cpu.execute(0, ins2) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
