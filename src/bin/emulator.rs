use clap::Parser;
use yayachip8rsemu::disasm::Instruction;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yayachip8rsemu::state::Chip8State;
use macroquad::prelude::*;
use std::time::SystemTime;
use std::{thread, time};

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

    /// Chip8 pixel size.
    #[arg(short, long, action, default_value_t = 16)]
    pixel_size: i32,
}

#[macroquad::main("yayachip8rsemu")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut file = File::open(args.file)?;
    let mut memory = vec![0u8; 0x200];
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;
    memory.append(&mut contents);
    let mut chip8_state = Chip8State::from_memory(memory);
    chip8_state.pc = 0x200;
    let mut now = SystemTime::now();
    chip8_state.memory.memory[0x1ff] = 3;

    let mut cycles = 0;
    loop {
        match now.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_millis() > 1000/60 {
                    clear_background(BLUE);
                    for (y, line) in chip8_state.screen.iter().enumerate() {
                        for (x, &pixel) in line.iter().enumerate() {
                            let ps = args.pixel_size;
                            if pixel {
                                draw_rectangle(
                                    x as f32*ps as f32,
                                    y as f32*ps as f32,
                                    ps as f32,
                                    ps as f32,
                                    GREEN
                                );
                            }
                        }
                    }
                    next_frame().await;
                    now = SystemTime::now();
                }
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
        let inst = chip8_state.memory.read(chip8_state.pc as usize, 2);
        let bytes = [inst[0], inst[1]];
        let word = u16::from_be_bytes(bytes);
        let instruction = Instruction::from(word);
        //println!("{:04x}:\t{:04x} {}", chip8_state.pc, word, instruction);
        chip8_state.execute_instruction();
        cycles += 1;

        let to_sleep = time::Duration::from_millis(10);
        let now = time::Instant::now();

        thread::sleep(to_sleep);
    }
}
