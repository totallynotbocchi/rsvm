pub mod vm;

use vm::*;

use crate::vm::{
    cpu::CPU,
    instruction::{Instruction, Opcode},
};

fn main() {
    let cpu = CPU::default();
    let ins = Instruction::new(Opcode::PrintReg);

    match cpu.execute(ins) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
