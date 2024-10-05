use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};
use crate::state::Chip8State;
use crate::args::Args;

fn print_ui_text(ui: &mut Ui, str: String) {
    for line in str.lines() {
        ui.label(None, line);
    }
}

fn debug_window_execution(
    ui: &mut Ui,
    chip8_state: &mut Chip8State,
    steps: &mut String,
    args: &Args
) {
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

    ui.tree_node(hash!(), "Step", |ui| {
        for step in [1, 10, 100] {
            if ui.button(None, format!("Step {}", step)) {
                chip8_state.step(step);
            }
            ui.same_line(0.0);
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
}

fn debug_window_breakpoints(
    ui: &mut Ui,
    chip8_state: &mut Chip8State,
    breakpoint_addr: &mut String,
) {
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
        // Not using an iterator since then I would have two borrows,
        // one mutable when removing
        for i in 0..chip8_state.breakpoints.len() {
            ui.label(None, &format!("{i:2}: {:#06x}", chip8_state.breakpoints[i].addr));
            ui.same_line(0.0);
            if ui.button(None, "Remove") {
                chip8_state.breakpoints.remove(i);
                // Break to avoid mismatched lengths after removing
                break;
            }
            ui.separator();
        }
    });
}

fn debug_window_speedhacks(
    ui: &mut Ui,
    chip8_state: &mut Chip8State,
    multiplier: &mut String,
) {
    ui.tree_node(hash!(), "Speedhacks", |ui| {
        for multiplier in [0.25, 1.0, 2.0, 4.0] {
            if ui.button(None, format!("{}x", multiplier)) {
                chip8_state.time_multiplier = multiplier;
            }
            ui.same_line(0.0);
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
}

fn debug_window_snapshots(
    ui: &mut Ui,
    chip8_state: &mut Chip8State,
    snapshots: &mut Vec<Chip8State>,
) {
    ui.tree_node(hash!(), "Snapshots", |ui| {
        if ui.button(None, "Add snapshot") {
            snapshots.push(chip8_state.clone());
        }
        ui.separator();
        ui.label(None, "Snapshots: ");
        
        for i in 0..snapshots.len() {
            ui.label(None, &format!("{:2}", i));
            ui.same_line(0.0);
            if ui.button(None, "Remove") {
                snapshots.remove(i);
                // Break to avoid mismatched lengths after removing
                break;
            }
            ui.same_line(0.0);
            if ui.button(None, "Load") {
                *chip8_state = snapshots[i].clone();
            }
            ui.separator();
        }
    });
}

fn debug_window(
    chip8_state: &mut Chip8State,
    snapshots: &mut Vec<Chip8State>,
    steps: &mut String,
    breakpoint_addr: &mut String,
    multiplier: &mut String,
    args: &Args
) {
    widgets::Window::new(hash!(), vec2(0., 50.), vec2(200., 300.))
        .label("Debug")
        .ui(&mut root_ui(), |ui| {
            debug_window_execution(ui, chip8_state, steps, args);
            debug_window_breakpoints(ui, chip8_state, breakpoint_addr);
            debug_window_speedhacks(ui, chip8_state, multiplier);
            debug_window_snapshots(ui, chip8_state, snapshots);
        });
}

fn registers_window(chip8_state: &Chip8State) {
    widgets::Window::new(hash!(), vec2(300., 50.), vec2(250., 300.))
        .label("State")
        .ui(&mut root_ui(), |ui| {
            print_ui_text(ui, chip8_state.get_state_string());
        });
}

fn disassembly_window(chip8_state: &Chip8State) {
    widgets::Window::new(hash!(), vec2(600., 50.), vec2(300., 300.))
        .label("Disassembly")
        .ui(&mut root_ui(), |ui| {
            print_ui_text(ui, chip8_state.get_disassembly_string());
        });
}

pub fn debug_windows(
    chip8_state: &mut Chip8State,
    snapshots: &mut Vec<Chip8State>,
    steps: &mut String,
    breakpoint_addr: &mut String,
    multiplier: &mut String,
    args: &Args
) {
    debug_window(chip8_state, snapshots, steps, breakpoint_addr, multiplier, args);
    registers_window(chip8_state);
    disassembly_window(chip8_state);
}
