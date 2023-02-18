use clap::Parser;
use yayachip8rsemu::state::Chip8State;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author = "tabun dareka <tabun.dareka@protonmail.com>")]
#[command(about = "Chip8 emulator", long_about = None)]
#[command(version)]
struct Args {
   /// Print verbose information.
   #[arg(short, long, action, default_value_t = false)]
   verbose: bool,

   /// File to run.
   #[arg(short, long)]
   file: String,

   /// Where to load the binary image.
   #[arg(short, long, action, default_value_t = 0x200)]
   offset: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut file = File::open(args.file)?;
    let mut memory = vec![0u8; 0x200];
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;
    memory.append(&mut contents);
    let mut chip8_state = Chip8State::from_memory(memory);
    chip8_state.pc = 0x200;
    loop {
        chip8_state.execute_instruction();
    }

    Ok(())
}
