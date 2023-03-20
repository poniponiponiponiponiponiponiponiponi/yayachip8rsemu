use std::mem::size_of;
use core::fmt::Display;
use std::ops::Shl;
use std::ops::BitOr;

pub struct Memory {
    pub memory: [u8; 4096],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 4096],
        }
    }

    pub fn from_vec(vec: Vec::<u8>) -> Memory {
        let mut memory = Memory::new();
        for (i, &byte) in vec.iter().enumerate() {
            memory.memory[i] = byte;
        }
        memory
    }

    pub fn write(&mut self, beg: usize, to_write: &[u8]) {
        for (i, &byte) in to_write.iter().enumerate() {
            self.memory[beg+i] = byte;
        }
    }

    pub fn read(&self, beg: usize, to_read: usize) -> Vec<u8> {
        let mut ret = Vec::new();
        for &byte in self.memory[beg..beg+to_read].iter() {
            ret.push(byte);
        }
        ret
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn read_t<T: Default + Shl<usize, Output = T> + BitOr<u8, Output = T> + Display>
        (&self, beg: usize) -> T {
        let size = size_of::<T>();
        let mut number: T = T::default();
        // This might seem like a weird way to do this and you're right - it is.
        // However I can't find a way to easily get rid of the 'attempt to shift
        // left with overflow' error with a generic...
        // There's the "num" crate but for some reasons it only provides traits for
        // numeric operations and not binary operations which is not helpful at all.
        let mut first_byte = true;
        for &byte in self.memory[beg..beg+size].iter() {
            if first_byte {
                number = number | byte;
                first_byte = false;
            } else {
                number = (number << 8) | byte;
            }
        }
        number
    }
}
