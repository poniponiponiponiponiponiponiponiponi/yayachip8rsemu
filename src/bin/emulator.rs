use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yayachip8rsemu::state::Chip8State;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};
use macroquad::audio::{load_sound, play_sound_once, PlaySoundParams};
use std::time::SystemTime;
use std::{thread, time};

#[derive(Parser, Debug)]
#[command(author = "poni <poniponiponiponiponiponiponiponiponiponi@protonmail.com>")]
#[command(about = "yayachip8rsemu", long_about = None)]
#[command(version)]
struct Args {
    /// File to run.
    #[arg(short, long)]
    file: String,

    /// Where to load the binary image.
    #[arg(short, long, action, default_value_t = 0x200)]
    offset: u16,

    /// Execution start address.
    #[arg(long, action, default_value_t = 0x200)]
    start: u16,

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
    let mut latest_press: usize = 0;
    let mut pressed: bool = false;

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

fn debug_windows(
        chip8_state: &mut Chip8State,
        steps: &mut String,
        breakpoint_addr: &mut String,
        multiplier: &mut String,
        args: &Args) {
    widgets::Window::new(hash!(), vec2(0., 50.), vec2(200., 300.))
        .label("Debug")
        .ui(&mut root_ui(), |ui| {
            if ui.button(None, "Stop") {
                chip8_state.stop_execution();
            }
            ui.same_line(0.0);
            if ui.button(None, "Continue") {
                chip8_state.continue_execution();
            }
            ui.same_line(0.0);
            if ui.button(None, "Restart") {
                *chip8_state = create_chip8_from_args(args).unwrap();
            }

            ui.separator();
            ui.tree_node(hash!(), "Step", |ui| {
                if ui.button(None, "Step 1") {
                    chip8_state.step(1);
                }
                ui.same_line(0.0);
                if ui.button(None, "Step 10") {
                    chip8_state.step(10);
                }
                ui.same_line(0.0);
                if ui.button(None, "Step 100") {
                    chip8_state.step(100);
                }
                ui.separator();
                ui.label(None, "Make X amount of steps: ");
                ui.input_text(hash!(), "< --", steps);
                ui.separator();
                if ui.button(None, "Step X") {
                    let steps = steps.parse::<u16>();
                    if let Ok(steps) = steps {
                        chip8_state.step(steps);
                    } else {
                        eprintln!("steps is not a number");
                    }
                }
            });

            ui.separator();
            ui.tree_node(hash!(), "Breakpoints", |ui| {
                ui.label(None, "Breakpoint address: ");
                ui.input_text(hash!(), "< --", breakpoint_addr);
                ui.separator();
                if ui.button(None, "Add breakpoint") {
                    let breakpoint_addr = breakpoint_addr.parse::<u16>();
                    if let Ok(breakpoint_addr) = breakpoint_addr {
                        chip8_state.add_breakpoint(breakpoint_addr);
                    } else {
                        eprintln!("breakpoint address is not an address");
                    }
                }
                ui.separator();
                ui.label(None, "Breakpoints: ");

                let mut to_remove = Vec::new();
                for (i, bp) in chip8_state.breakpoints.iter().enumerate() {
                    ui.label(None, &format!("{i:2}: {:#06x}", bp.addr));
                    ui.same_line(0.0);
                    if ui.button(None, "Remove") {
                        let idx = chip8_state.breakpoints
                            .iter()
                            .position(|x| x == bp)
                            .unwrap();
                        to_remove.push(idx);
                    }
                    ui.separator();
                }
                to_remove.sort();
                to_remove.reverse();
                for idx in to_remove {
                    chip8_state.breakpoints.remove(idx);
                }
            });

            ui.separator();
            ui.tree_node(hash!(), "Speedhacks", |ui| {
                if ui.button(None, "0.25x") {
                    chip8_state.time_multiplier = 0.25;
                }
                ui.same_line(0.0);
                if ui.button(None, "1x   ") {
                    chip8_state.time_multiplier = 1.0;
                }
                ui.same_line(0.0);
                if ui.button(None, "2x   ") {
                    chip8_state.time_multiplier = 2.0;
                }
                ui.same_line(0.0);
                if ui.button(None, "4x   ") {
                    chip8_state.time_multiplier = 4.0;
                }
                ui.separator();
                ui.label(None, "Custom time multiplier: ");
                ui.input_text(hash!(), "< --", multiplier);
                ui.separator();
                if ui.button(None, "Apply") {
                    let multiplier = multiplier.parse::<f64>();
                    if let Ok(multiplier) = multiplier {
                        chip8_state.time_multiplier = multiplier;
                    } else {
                        eprintln!("Multiplier is not a number");
                    }
                }
            });
        });

    widgets::Window::new(hash!(), vec2(300., 50.), vec2(250., 300.))
        .label("State")
        .ui(&mut root_ui(), |ui| {
            print_ui_text(ui, chip8_state.get_state_string());
        });

    widgets::Window::new(hash!(), vec2(600., 50.), vec2(300., 300.))
        .label("Disassembly")
        .ui(&mut root_ui(), |ui| {
            print_ui_text(ui, chip8_state.get_disassembly_string());
        });
}

async fn main_loop(chip8_state: &mut Chip8State, args: &Args) {
    let mut screen_timer = SystemTime::now();
    let mut timer_timer = SystemTime::now();

    let sound = load_sound("./sound.ogg").await;
    if let Err(_) = sound {
        eprintln!("Error while loading sound file");
    }

    // input values for the interface
    let mut steps = String::new();
    let mut breakpoint_addr = String::new();
    let mut multiplier = String::new();
    loop {
        handle_input(chip8_state);
        if chip8_state.sound_timer > 0 {
            if let Ok(sound) = sound {
                play_sound_once(sound);
            }
        }

        match timer_timer.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_millis() > (1000 as f64/60 as f64/chip8_state.time_multiplier) as u128 {
                    if chip8_state.delay_timer != 0 {
                        chip8_state.delay_timer -= 1;
                    }
                    if chip8_state.sound_timer != 0 {
                        chip8_state.sound_timer -= 1;
                    }
                    timer_timer = SystemTime::now();
                }
            }
            Err(e) => {
                eprintln!("Error: {e:?}");
            }
        }

        match screen_timer.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_millis() > 1000/60 {
                    // drawing
                    draw_screen(chip8_state, args.pixel_size as usize);
                    if args.debug_mode {
                        debug_windows(chip8_state, &mut steps, &mut breakpoint_addr, &mut multiplier, args);
                    }
                    next_frame().await;

                    screen_timer = SystemTime::now();
                }
            }
            Err(e) => {
                eprintln!("Error: {e:?}");
            }
        }
        // handle emulation
        chip8_state.emulate_instruction();

        let to_sleep = time::Duration::from_micros((2000.0/chip8_state.time_multiplier) as u64);

        thread::sleep(to_sleep);
    }
}

fn create_chip8_from_args(args: &Args) -> Result<Chip8State, Box<dyn Error>> {
    let mut file = File::open(&args.file)?;
    let mut memory = vec![0u8; 0x200];
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;
    memory.append(&mut contents);
    let mut chip8_state = Chip8State::from_memory(memory);
    chip8_state.pc = args.start;
    Ok(chip8_state)
}

#[macroquad::main("yayachip8rsemu")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut chip8_state = create_chip8_from_args(&args)?;

    main_loop(&mut chip8_state, &args).await;

    Ok(())
}
