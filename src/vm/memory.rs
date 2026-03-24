use crate::error::Error;

// ----------- STACK MEMORY -----------

// TODO: make it point to a region in RAM instead
pub struct Stack {
    data: Vec<i32>,
    capacity: usize,
    ptr: usize,
}

impl Stack {
    const DEFAULT_SIZE: usize = 256;

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

impl Default for Stack {
    fn default() -> Self {
        Self {
            data: vec![],
            capacity: Stack::DEFAULT_SIZE,
            ptr: 0,
        }
    }
}

// ----------- RAM MEMORY -----------

pub struct RAM {
    data: Vec<u8>,
    size: usize,
}

impl RAM {
    const DEFAULT_SIZE: usize = 2 ^ 12;

    pub fn new(size: usize) -> Self {
        Self { data: vec![], size }
    }

    pub fn get_at(&self, addr: usize) -> Result<u8, Error> {
        if addr >= self.size {
            return Err(Error::OutOfBoundsMemoryAccess);
        }

        match self.data.get(addr) {
            Some(v) => Ok(v.clone()),
            None => Err(Error::InvalidMemoryAccess),
        }
    }

    pub fn set_at(&mut self, addr: usize, value: u8) -> Result<(), Error> {
        if addr >= self.size {
            return Err(Error::OutOfBoundsMemoryAccess);
        }

        self.data[addr] = value;
        Ok(())
    }
}

impl Default for RAM {
    fn default() -> Self {
        Self {
            size: RAM::DEFAULT_SIZE,
            data: Vec::default(),
        }
    }
}
