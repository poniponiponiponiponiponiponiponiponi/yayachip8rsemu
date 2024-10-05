use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result as IoResult;
use yayachip8rsemu::state::Chip8State;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};
use macroquad::audio::{load_sound, play_sound_once};
use std::time::SystemTime;
use std::{thread, time};

#[derive(Parser, Debug)]
#[command(author = "poni <poniponiponiponiponiponiponiponiponiponi@protonmail.com>")]
#[command(about = "yayachip8rsemu", long_about = None)]
#[command(version)]
struct Args {
    /// ROM to run
    #[arg(short, long)]
    file: String,

    /// Offset where to load the binary image in the Chip8 address space
    #[arg(short, long, action, default_value_t = 0x200)]
    offset: u16,

    /// Start address of the execution
    #[arg(long, action, default_value_t = 0x200)]
    start: u16,

    /// Pixel size
    #[arg(short, long, action, default_value_t = 16)]
    pixel_size: i32,

    /// Start with stopped execution
    #[arg(short, long, action, default_value_t = false)]
    stop: bool,

    /// Debug mode. Draw special debug windows
    #[arg(short, long, action, default_value_t = false)]
    debug_mode: bool,
}

impl Args {
    fn create_chip8(&self) -> IoResult<Chip8State> {
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
    
    for (keyboard_key, chip8_key) in keyboard_key_chip8_key_pairs {
        let is_pressed = is_key_down(keyboard_key);
        if is_pressed && !chip8_state.key_pressed[chip8_key] {
            // last pressed key
            chip8_state.reg[chip8_state.keypress_reg as usize] = chip8_key as u8;
        }
        chip8_state.key_pressed[chip8_key] = is_pressed;
    }
}

fn print_ui_text(ui: &mut Ui, str: String) {
    for line in str.lines() {
        ui.label(None, line);
    }
}

fn draw_screen(chip8_state: &mut Chip8State, ps: usize) {
    clear_background(DARKGRAY);
    let mut last_pixel = (0, 0);
    for (y, line) in chip8_state.screen.iter().enumerate() {
        for (x, &pixel) in line.iter().enumerate() {
            last_pixel = (x, y);
            if pixel {
                draw_rectangle(
                    x as f32 * ps as f32,
                    y as f32 * ps as f32,
                    ps as f32,
                    ps as f32,
                    PURPLE
                );
            }
        }
        
        // Draw a line on the right side of the screen to designate
        // an end of the screen
        draw_rectangle(
            (last_pixel.0 + 1) as f32 * ps as f32,
            last_pixel.1 as f32 * ps as f32,
            ps as f32,
            ps as f32,
            GRAY
        );
    }
    
    // Draw a line on the bottom of the screen
    for x in 0..=last_pixel.0+1 {
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
                match args.create_chip8() {
                    Ok(state) => {
                        *chip8_state = state;
                    },
                    Err(e) => {
                        eprintln!("Unexpected error: {}.\nQuitting...", e);
                        std::process::exit(1);
                    }
                }
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
    // Later used so the excess in case when time_diff > 0 (instead of
    // timer_diff == 0) doesn't go to waste
    let mut screen_excess = 0.0;
    let mut timer_excess = 0.0;

    let sound = load_sound("./sound.ogg").await;
    if let Err(e) = &sound {
        eprintln!("Error while loading sound file: {}", e);
    }

    // Input variables for the debug windows
    let mut steps = String::new();
    let mut breakpoint_addr = String::new();
    let mut multiplier = String::new();
    // So called main execution loop
    loop {
        // Hanlde input
        handle_input(chip8_state);

        // Handle all timers
        // Sound timer
        if chip8_state.sound_timer > 0 {
            if let Ok(sound) = sound {
                play_sound_once(sound);
            }
        }
        // Update inner CHIP-8 timers every 1/60 second as defined in
        // the technical reference
        match timer_timer.elapsed() {
            Ok(elapsed) => {
                let secs = elapsed.as_secs_f64();
                let time_diff = secs + timer_excess - (1.0 / 60.0 / chip8_state.time_multiplier);
                if time_diff >= 0.0 {
                    if chip8_state.delay_timer != 0 {
                        chip8_state.delay_timer -= 1;
                    }
                    if chip8_state.sound_timer != 0 {
                        chip8_state.sound_timer -= 1;
                    }
                    timer_excess = time_diff;
                    timer_timer = SystemTime::now();
                }
            }
            Err(e) => {
                eprintln!("Timer's timer error: {e:?}");
            }
        }
        
        // Update screen every 1/60 second so we have 60 fps. Not
        // defined in the technical reference
        match screen_timer.elapsed() {
            Ok(elapsed) => {
                let secs = elapsed.as_secs_f64();
                let time_diff = secs + screen_excess - (1.0 / 60.0);
                if time_diff >= 0.0 {
                    draw_screen(chip8_state, args.pixel_size as usize);
                    if args.debug_mode {
                        debug_windows(
                            chip8_state,
                            &mut steps,
                            &mut breakpoint_addr,
                            &mut multiplier,
                            args
                        );
                    }
                    next_frame().await;
                    screen_excess = time_diff;
                    screen_timer = SystemTime::now();
                }
            }
            Err(e) => {
                eprintln!("Screen's timer error: {e:?}");
            }
        }
        
        // Handle emulation
        chip8_state.emulate_instruction();

        // CHIP-8 doesn't really have a set cpu frequency but
        // according to a random reddit post some ROMs might be
        // frequency sensitive, so a cpu clock around 2000hz seems
        // like a nice middleground
        // https://www.reddit.com/r/EmuDev/comments/gvmk12/comment/fsq9p8a/
        let to_sleep = time::Duration::from_secs_f64(1.0/2000.0/chip8_state.time_multiplier);
        thread::sleep(to_sleep);
    }
}

#[macroquad::main("yayachip8rsemu")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut chip8_state = args.create_chip8()?;

    main_loop(&mut chip8_state, &args).await;

    Ok(())
}
