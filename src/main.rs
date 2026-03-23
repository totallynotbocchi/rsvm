pub mod vm;

use vm::*;

use crate::vm::instruction::{Instruction, Opcode};

fn main() {
    let ins = Instruction::new(Opcode::PrintReg);

    let test: Instruction;
    match Instruction::decoded(ins.packed()) {
        Ok(v) => test = v,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    }

    println!("{:?}", test);
    println!("ORIGINAL {:?}", ins);
    println!("PACKED: {:#010x}", test.packed());
}
