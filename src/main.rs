pub mod vm;

use vm::*;

use crate::vm::{
    cpu::CPU,
    instruction::{Instruction, Opcode, Operand},
};

fn main() {
    let mut cpu = CPU::default();

    let ins1 = Instruction::new(
        Opcode::Put,
        Operand::Intermediate,
        Operand::None,
        Operand::Register(register::Register::G0),
    )
    .packed()
    .unwrap();

    cpu.ram
        .set_at(0, ins1)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    cpu.ram
        .set_at(1, 2)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    let ins2 = Instruction::new(
        Opcode::Put,
        Operand::Intermediate,
        Operand::None,
        Operand::Register(register::Register::G1),
    )
    .packed()
    .unwrap();

    cpu.ram
        .set_at(2, ins2)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    cpu.ram
        .set_at(3, 2)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    let ins3 = Instruction::new(
        Opcode::Add,
        Operand::Register(register::Register::G0),
        Operand::Register(register::Register::G1),
        Operand::Register(register::Register::G3),
    )
    .packed()
    .unwrap();

    cpu.ram
        .set_at(4, ins3)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    let ins4 = Instruction::new(
        Opcode::PrintReg,
        Operand::Register(register::Register::G3),
        Operand::None,
        Operand::None,
    )
    .packed()
    .unwrap();

    cpu.ram
        .set_at(5, ins4)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    let ins5 = Instruction::new(Opcode::Exit, Operand::None, Operand::None, Operand::None)
        .packed()
        .unwrap();

    cpu.ram
        .set_at(6, ins5)
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    cpu.main_loop()
        .unwrap_or_else(|e| eprintln!("Main loop error: {:?}", e));
}
