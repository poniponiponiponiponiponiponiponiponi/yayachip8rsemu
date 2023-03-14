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

    loop {
        // handle keys
        let mut latest_press: usize = 0;
        let mut pressed: bool = false;
        let is_pressed = is_key_down(KeyCode::Key1);
        if is_pressed && chip8_state.key_pressed[0x1] == false {
            latest_press = 0x1;
            pressed = true;
        }
        chip8_state.key_pressed[0x1] = is_pressed;

        let is_pressed = is_key_down(KeyCode::Key2);
        if is_pressed && chip8_state.key_pressed[0x2] == false {
            latest_press = 0x2;
            pressed = true;
        }
        chip8_state.key_pressed[0x2] = is_pressed;

        let is_pressed = is_key_down(KeyCode::Key3);
        if is_pressed && chip8_state.key_pressed[0x3] == false {
            latest_press = 0x3;
            pressed = true;
        }
        chip8_state.key_pressed[0x3] = is_pressed;

        let is_pressed = is_key_down(KeyCode::Key4);
        if is_pressed && chip8_state.key_pressed[0xc] == false {
            latest_press = 0xc;
            pressed = true;
        }
        chip8_state.key_pressed[0xc] = is_pressed;

        let is_pressed = is_key_down(KeyCode::Q);
        if is_pressed && chip8_state.key_pressed[0x4] == false {
            latest_press = 0x4;
            pressed = true;
        }
        chip8_state.key_pressed[0x4] = is_pressed;

        let is_pressed = is_key_down(KeyCode::W);
        if is_pressed && chip8_state.key_pressed[0x5] == false {
            latest_press = 0x5;
            pressed = true;
        }
        chip8_state.key_pressed[0x5] = is_pressed;

        let is_pressed = is_key_down(KeyCode::E);
        if is_pressed && chip8_state.key_pressed[0x6] == false {
            latest_press = 0x6;
            pressed = true;
        }
        chip8_state.key_pressed[0x6] = is_pressed;

        let is_pressed = is_key_down(KeyCode::R);
        if is_pressed && chip8_state.key_pressed[0xd] == false {
            latest_press = 0xd;
            pressed = true;
        }
        chip8_state.key_pressed[0xd] = is_pressed;

        let is_pressed = is_key_down(KeyCode::A);
        if is_pressed && chip8_state.key_pressed[0x7] == false {
            latest_press = 0x7;
            pressed = true;
        }
        chip8_state.key_pressed[0x7] = is_pressed;

        let is_pressed = is_key_down(KeyCode::S);
        if is_pressed && chip8_state.key_pressed[0x8] == false {
            latest_press = 0x8;
            pressed = true;
        }
        chip8_state.key_pressed[0x8] = is_pressed;

        let is_pressed = is_key_down(KeyCode::D);
        if is_pressed && chip8_state.key_pressed[0x9] == false {
            latest_press = 0x9;
            pressed = true;
        }
        chip8_state.key_pressed[0x9] = is_pressed;

        let is_pressed = is_key_down(KeyCode::F);
        if is_pressed && chip8_state.key_pressed[0xe] == false {
            latest_press = 0xe;
            pressed = true;
        }
        chip8_state.key_pressed[0xe] = is_pressed;

        let is_pressed = is_key_down(KeyCode::Z);
        if is_pressed && chip8_state.key_pressed[0xa] == false {
            latest_press = 0xa;
            pressed = true;
        }
        chip8_state.key_pressed[0xa] = is_pressed;

        let is_pressed = is_key_down(KeyCode::X);
        if is_pressed && chip8_state.key_pressed[0x0] == false {
            latest_press = 0x0;
            pressed = true;
        }
        chip8_state.key_pressed[0x0] = is_pressed;

        let is_pressed = is_key_down(KeyCode::C);
        if is_pressed && chip8_state.key_pressed[0xb] == false {
            latest_press = 0xb;
            pressed = true;
        }
        chip8_state.key_pressed[0xb] = is_pressed;

        let is_pressed = is_key_down(KeyCode::V);
        if is_pressed && chip8_state.key_pressed[0xf] == false {
            latest_press = 0xf;
            pressed = true;
        }
        chip8_state.key_pressed[0xf] = is_pressed;

        if chip8_state.keypress_halt && pressed {
            chip8_state.keypress_halt = false;
            chip8_state.reg[chip8_state.keypress_reg as usize] = latest_press as u8;
        }

        // handle drawing
        match now.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_millis() > 1000/60 {
                    if chip8_state.delay_timer != 0 {
                        chip8_state.delay_timer -= 1;
                    }
                    if chip8_state.sound_timer != 0 {
                        chip8_state.sound_timer -= 1;
                    }
                    clear_background(BLACK);
                    let ps = args.pixel_size;
                    let mut last_pixel = (0, 0);
                    for (y, line) in chip8_state.screen.iter().enumerate() {
                        for (x, &pixel) in line.iter().enumerate() {
                            last_pixel = (x, y);
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
                        draw_rectangle(
                            last_pixel.0 as f32*ps as f32,
                            last_pixel.1 as f32*ps as f32,
                            ps as f32,
                            ps as f32,
                            GRAY
                        );
                    }
                    for x in 0..=last_pixel.0 {
                        draw_rectangle(
                            x as f32*ps as f32,
                            (last_pixel.1 + 1) as f32*ps as f32,
                            ps as f32,
                            ps as f32,
                            GRAY
                        );
                    }
                    next_frame().await;
                    now = SystemTime::now();
                }
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
        // handle emulation
        // let inst = chip8_state.memory.read(chip8_state.pc as usize, 2);
        // let bytes = [inst[0], inst[1]];
        // let word = u16::from_be_bytes(bytes);
        // let instruction = Instruction::from(word);
        //println!("{:04x}:\t{:04x} {}", chip8_state.pc, word, instruction);
        if !chip8_state.keypress_halt {
            chip8_state.execute_instruction();
        }

        let to_sleep = time::Duration::from_millis(4);

        thread::sleep(to_sleep);
    }
}
