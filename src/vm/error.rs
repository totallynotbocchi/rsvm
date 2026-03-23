#[derive(Debug)]
pub enum Error {
    StackOverflow,
    StackUnderflow,
    InvalidOpcode,
}
