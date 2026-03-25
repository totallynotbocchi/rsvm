pub mod vm;

use vm::*;

use crate::vm::{
    cpu::CPU,
    instruction::{Instruction, Opcode, Operand},
};

fn main() {
    let mut cpu = CPU::default();

    cpu.ram
        .set_at(1, 2)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    cpu.ram
        .set_at(2, 4)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    let ins1 = Instruction::new(
        Opcode::Put,
        Operand::Intermediate,
        Operand::None,
        Operand::Register(register::Register::G0),
    );

    let ins2 = Instruction::new(
        Opcode::Put,
        Operand::Intermediate,
        Operand::None,
        Operand::Register(register::Register::G1),
    );

    let ins3 = Instruction::new(
        Opcode::Add,
        Operand::Register(register::Register::G0),
        Operand::Register(register::Register::G1),
        Operand::Register(register::Register::G3),
    );

    let ins4 = Instruction::new(
        Opcode::PrintReg,
        Operand::Register(register::Register::G3),
        Operand::None,
        Operand::None,
    );

    cpu.execute(0, ins1)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    cpu.execute(1, ins2)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    cpu.execute(0, ins3)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    cpu.execute(0, ins4)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));
}
