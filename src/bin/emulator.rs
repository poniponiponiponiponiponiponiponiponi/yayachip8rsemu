use yayachip8rsemu::state::{Chip8State, QuirksConfig};
use yayachip8rsemu::args::Args;
use yayachip8rsemu::debug;
use clap::{Parser, ValueEnum};
use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound_once};
use std::time::SystemTime;
use std::{thread, time};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author = "poni <poniponiponiponiponiponiponiponiponiponi@protonmail.com>")]
#[command(about = "yayachip8rsemu", long_about = None)]
#[command(version)]
struct Cli {
    /// ROM to run
    #[arg(short, long)]
    file: String,

    /// Offset where to load the binary image in the CHIP-8 address space
    #[arg(short, long, default_value_t = 0x200)]
    offset: u16,

    /// Start address of the execution
    #[arg(long, default_value_t = 0x200)]
    start: u16,

    /// Pixel size
    #[arg(short, long, default_value_t = 16)]
    pixel_size: i32,

    /// Start with stopped execution
    #[arg(short, long, default_value_t = false)]
    stop: bool,

    /// Debug mode. Draw special debug windows
    #[arg(short, long, default_value_t = false)]
    debug_mode: bool,

    /// Pick quirks
    #[arg(value_enum, short, long, default_value_t = Chip8Quirks::Chip8)]
    quirks: Chip8Quirks
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Chip8Quirks {
    Chip8,
    SuperChip,
    XoChip
}

impl Cli {
    fn to_args(&self) -> Args {
        Args {
            file: self.file.clone(),
            offset: self.offset,
            start: self.start,
            pixel_size: self.pixel_size,
            stop: self.stop,
            debug_mode: self.debug_mode,
            quirks_config: match self.quirks {
                Chip8Quirks::Chip8 => {
                    QuirksConfig::get_chip8()
                },
                Chip8Quirks::XoChip => {
                    QuirksConfig::get_xo_chip()
                },
                Chip8Quirks::SuperChip => {
                    QuirksConfig::get_super_chip()
                }
            }
        }
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

    // Variables for the debug windows
    let mut steps = String::new();
    let mut breakpoint_addr = String::new();
    let mut multiplier = String::new();
    let mut snapshots = Vec::new();
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
                        debug::debug_windows(
                            chip8_state,
                            &mut snapshots,
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
        // frequency sensitive, so a cpu clock around 1000hz seems
        // like a nice middleground
        // https://www.reddit.com/r/EmuDev/comments/gvmk12/comment/fsq9p8a/
        let to_sleep = time::Duration::from_secs_f64(1.0/1000.0/chip8_state.time_multiplier);
        thread::sleep(to_sleep);
    }
}

#[macroquad::main("yayachip8rsemu")]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let args = cli.to_args();
    let mut chip8_state = args.create_chip8()?;

    main_loop(&mut chip8_state, &args).await;

    Ok(())
}
