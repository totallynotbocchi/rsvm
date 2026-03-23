pub mod vm;

use vm::*;

use crate::vm::register::Register;

fn main() {
    let mut vm = VM::default();

    vm.set_register(Register::G0, 2);
    println!("{}", vm.get_register(Register::G0));
}
