pub mod vm;

use vm::*;

fn main() {
    let mut s = stack::Stack::new(1);

    if let Err(_) = s.push(2) {
        println!("Failed to add 2");
    }

    if let Err(_) = s.push(3) {
        println!("Failed to add 3");
    }

    match s.pop() {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{:?}", err),
    };
}
