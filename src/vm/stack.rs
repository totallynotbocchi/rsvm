use crate::error::Error;

pub struct Stack {
    data: Vec<i32>,
    capacity: usize,
    ptr: usize,
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![],
            capacity,
            ptr: 0,
        }
    }

    pub fn push(&mut self, value: i32) -> Result<(), Error> {
        if self.ptr + 1 > self.capacity {
            return Err(Error::StackOverflow);
        }

        self.ptr += 1;
        self.data.push(value);

        Ok(())
    }

    pub fn pop(&mut self) -> Result<i32, Error> {
        match self.data.pop() {
            Some(v) => {
                self.ptr -= 1;
                Ok(v)
            }

            None => Err(Error::StackUnderflow),
        }
    }
}
