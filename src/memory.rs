use std::mem::size_of;
use std::ops::Shl;
use std::ops::BitAnd;

pub struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 4096],
        }
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

    pub fn read_t<T: Default + Shl<usize, Output = T> + BitAnd<u8, Output = T>>
        (&self, beg: usize) {
        let size = size_of::<T>();
        let mut number: T = T::default();
        for &byte in self.memory[beg..beg+size].iter() {
            number = (number << 8) & byte;
        }
    }
}
