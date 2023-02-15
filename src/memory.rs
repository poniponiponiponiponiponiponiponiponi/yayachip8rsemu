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
            ret.push(byte)
        }
        ret
    }
}
