use std::fs::File;
use std::io::prelude::*;
use std::io::Result as IoResult;
use crate::state::Chip8State;

pub struct Args {
    pub file: String,
    pub offset: u16,
    pub start: u16,
    pub pixel_size: i32,
    pub stop: bool,
    pub debug_mode: bool,
}

impl Args {
    pub fn create_chip8(&self) -> IoResult<Chip8State> {
        let mut file = File::open(&self.file)?;
        let mut memory = vec![0u8; 0x200];
        let mut contents = Vec::<u8>::new();
        file.read_to_end(&mut contents)?;
        memory.append(&mut contents);
        let mut chip8_state = Chip8State::from_memory(memory);
        chip8_state.pc = self.start;
        Ok(chip8_state)
    }
}
