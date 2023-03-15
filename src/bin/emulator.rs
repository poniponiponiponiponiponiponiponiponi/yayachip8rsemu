use clap::Parser;
// use yayachip8rsemu::disasm::Instruction;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yayachip8rsemu::state::Chip8State;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};
use std::time::SystemTime;
use std::{thread, time};

#[derive(Parser, Debug)]
#[command(author = "tabun dareka <tabun.dareka@protonmail.com>")]
#[command(about = "Chip8 emulator", long_about = None)]
#[command(version)]
struct Args {
    /// File to run.
    #[arg(short, long)]
    file: String,

    /// Where to load the binary image.
    #[arg(short, long, action, default_value_t = 0x200)]
    offset: usize,

    /// Chip8 pixel size.
    #[arg(short, long, action, default_value_t = 16)]
    pixel_size: i32,

    /// Start with stopped execution.
    #[arg(short, long, action, default_value_t = false)]
    stop: bool,

    /// Debug mode. Draw special debug windows.
    #[arg(short, long, action, default_value_t = false)]
    debug_mode: bool,
}

fn handle_input(chip8_state: &mut Chip8State) {
    let mut latest_press: usize = 0;
    let mut pressed: bool = false;
    let keyboard_key_chip8_key_pairs = [
        (KeyCode::Key1, 0x1),
        (KeyCode::Key2, 0x2),
        (KeyCode::Key3, 0x3),
        (KeyCode::Key4, 0xc),
        (KeyCode::Q, 0x4),
        (KeyCode::W, 0x5),
        (KeyCode::E, 0x6),
        (KeyCode::R, 0xd),
        (KeyCode::A, 0x7),
        (KeyCode::S, 0x8),
        (KeyCode::D, 0x9),
        (KeyCode::F, 0xe),
        (KeyCode::Z, 0xa),
        (KeyCode::X, 0x0),
        (KeyCode::C, 0xb),
        (KeyCode::V, 0xf),
    ];
    for (keyboard_key, chip8_key) in keyboard_key_chip8_key_pairs {
        let is_pressed = is_key_down(keyboard_key);
        if is_pressed && !chip8_state.key_pressed[chip8_key] {
            latest_press = chip8_key;
            pressed = true;
        }
        chip8_state.key_pressed[chip8_key] = is_pressed;
    }

    if chip8_state.keypress_halt && pressed {
        chip8_state.keypress_halt = false;
        chip8_state.reg[chip8_state.keypress_reg as usize] = latest_press as u8;
    }
}

fn print_ui_text(ui: &mut Ui, str: String) {
    for line in str.lines() {
        ui.label(None, line);
    }
}

fn draw_screen(chip8_state: &mut Chip8State, ps: usize) {
    clear_background(BLACK);
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
                    LIME
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
}

fn draw_debug_windows(chip8_state: &mut Chip8State) {
    let mut data0: String = "".to_string();
    widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("Debug")
        .ui(&mut root_ui(), |ui| {
            ui.label(None, "Some random text");
            if ui.button(None, "Stop") {
            }
            ui.same_line(0.0);
            if ui.button(None, "Start") {
            }

            ui.separator();
            if ui.button(None, "Step 1") {
            }
            ui.same_line(0.0);
            if ui.button(None, "Step 10") {
            }
            ui.same_line(0.0);
            if ui.button(None, "Step 100") {
            }
            ui.separator();
            ui.label(None, "Make X amount of steps: ");
            ui.input_text(hash!(), "< --", &mut data0);
            ui.separator();
            if ui.button(None, "Step X") {
            }
        });
    widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("State")
        .ui(&mut root_ui(), |ui| {
            print_ui_text(ui, "AAAAAAAAAAAAAAAA\nBBBBBBBBBBBBBB".to_string());
        });
    widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("Disassembly")
        .ui(&mut root_ui(), |ui| {
            print_ui_text(ui, "AAAAAAAAAAAAAAAA\nBBBBBBBBBBBBBB".to_string());
        });
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
        handle_input(&mut chip8_state);
        match now.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_millis() > 1000/60 {
                    if chip8_state.delay_timer != 0 {
                        chip8_state.delay_timer -= 1;
                    }
                    if chip8_state.sound_timer != 0 {
                        chip8_state.sound_timer -= 1;
                    }

                    // drawing
                    draw_screen(&mut chip8_state, args.pixel_size as usize);
                    if args.debug_mode {
                        draw_debug_windows(&mut chip8_state);
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

        let to_sleep = time::Duration::from_millis(2);

        thread::sleep(to_sleep);
    }
}
