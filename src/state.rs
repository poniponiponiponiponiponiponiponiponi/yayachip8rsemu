use std::cmp;
use crate::disasm::Instruction;
use crate::stack::Stack;
use crate::memory::Memory;
use rand::Rng;

pub struct Breakpoint {
    pub addr: u16,
}

impl Breakpoint {
    pub fn new(addr: u16) -> Breakpoint {
        Breakpoint {
            addr
        }
    }
}

impl cmp::PartialEq for Breakpoint {
    fn eq(&self, other: &Self) -> bool {
        return self.addr == other.addr;
    }
}

pub struct Chip8State {
    pub pc: u16,
    // V0 to VF
    pub reg: [u8; 16],
    pub key_pressed: [bool; 16],
    // The I address register
    pub addr: u16,
    pub stack: Stack,
    pub memory: Memory,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub screen: [[bool; 64]; 32],
    pub keypress_halt: bool,
    pub keypress_reg: u8,
    pub stop: bool,
    pub steps_to_stop: u16,
    pub breakpoints: Vec<Breakpoint>,
    pub time_multiplier: f64,
}

impl Default for Chip8State {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8State {
    pub fn new() -> Chip8State {
        Chip8State {
            pc: 0,
            reg: [0; 16],
            key_pressed: [false; 16],
            addr: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            delay_timer: 0,
            sound_timer: 0,
            screen: [[false; 64]; 32],
            keypress_halt: false,
            keypress_reg: 0,
            stop: false,
            steps_to_stop: 0,
            breakpoints: Vec::new(),
            time_multiplier: 1.0,
        }
    }

    pub fn from_memory(memory: Vec<u8>) -> Chip8State {
        Chip8State {
            pc: 0,
            reg: [0; 16],
            key_pressed: [false; 16],
            addr: 0,
            stack: Stack::new(),
            memory: Memory::from_vec(memory),
            delay_timer: 0,
            sound_timer: 0,
            screen: [[false; 64]; 32],
            keypress_halt: false,
            keypress_reg: 0,
            stop: false,
            steps_to_stop: 0,
            breakpoints: Vec::new(),
            time_multiplier: 1.0,
        }
    }

    pub fn step(&mut self, steps: u16) {
        self.stop = false;
        self.steps_to_stop += steps;
    }

    pub fn stop_execution(&mut self) {
        self.stop = true;
        self.steps_to_stop = 0;
    }

    pub fn continue_execution(&mut self) {
        self.stop = false;
        self.steps_to_stop = 0;
    }

    pub fn check_for_breakpoints(&mut self) {
        for bp in self.breakpoints.iter() {
            if bp.addr == self.pc {
                self.stop = true;
                self.steps_to_stop = 0;
                break;
            }
        }
    }

    pub fn get_state_string(&self) -> String {
        let mut state_str = String::new();
        state_str += &format!("pc: {0:}\n", self.pc);
        for (i, val) in self.reg.iter().enumerate() {
            state_str += &format!("V{0:x}: {1:3} {1:#04x}", i, val);
            state_str += if i % 2 == 0 { "  |  " } else { "\n" };
        }
        state_str += &format!("I: {0:#06x}\n", self.addr);
        state_str += &format!("delay_timer: {:3}\n", self.delay_timer);
        state_str += &format!("sound_timer: {:3}\n", self.sound_timer);
        state_str
    }

    pub fn get_disassembly_string(&self) -> String {
        let mut disasm_str = String::new();
        for i in (-6..=18).step_by(2) {
            let inst_addr = self.pc as i32 + i;
            if inst_addr < 0 || inst_addr as usize >= self.memory.len() - 1 {
                disasm_str += "\n";
                continue;
            }
            let inst = self.memory.read(inst_addr as usize, 2);
            let bytes = [inst[0], inst[1]];
            let word = u16::from_be_bytes(bytes);
            let instruction = Instruction::from(word);
            if i == 0 {
                disasm_str += "--->  ";
            }
            disasm_str += &format!("{:04x}:\t{:04x} {}\n", inst_addr, word, instruction);
        }
        disasm_str
    }

    pub fn load_memory(&mut self, to_load: Vec<u8>, offset: usize) {
        for (i, &byte) in to_load.iter().enumerate() {
            self.memory.memory[i+offset] = byte;
        }
    }

    pub fn add_breakpoint(&mut self, breakpoint_addr: u16) {
        let bp = Breakpoint::new(breakpoint_addr);
        self.breakpoints.push(bp);
    }

    pub fn emulate_instruction(&mut self) {
        if !self.keypress_halt && !self.stop {
            self.execute_instruction();

            if self.steps_to_stop > 0 {
                self.steps_to_stop -= 1;
                if self.steps_to_stop == 0 {
                    self.stop = true;
                }
            }

            self.check_for_breakpoints();
        }
    }

    pub fn execute_instruction(&mut self) {
        let inst = self.memory.read(self.pc as usize, 2);
        let bytes = [inst[0], inst[1]];
        let inst = u16::from_be_bytes(bytes);
        Chip8State::find_instruction_func(inst)(self, inst);
    }

    pub fn find_instruction_func(inst: u16) -> fn(&mut Chip8State, u16) {
        if inst == 0x00e0 {
            Chip8State::clear_display
        } else if inst == 0x00ee {
            Chip8State::return_from_subroutine
        } else if inst & 0xf000 == 0x0000 {
            Chip8State::call_rca1802_code_routine
        } else if inst & 0xf000 == 0x1000 {
            Chip8State::jmp
        } else if inst & 0xf000 == 0x2000 {
            Chip8State::call
        } else if inst & 0xf000 == 0x3000 {
            Chip8State::skip_eq
        } else if inst & 0xf000 == 0x4000 {
            Chip8State::skip_neq
        } else if inst & 0xf000 == 0x5000 && inst & 0x000f == 0x0 {
            Chip8State::skip_regs_eq
        } else if inst & 0xf000 == 0x6000 {
            Chip8State::set_val
        } else if inst & 0xf000 == 0x7000 {
            Chip8State::add_val
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x0 {
            Chip8State::set_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x1 {
            Chip8State::or_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x2 {
            Chip8State::and_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x3 {
            Chip8State::xor_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x4 {
            Chip8State::add_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x5 {
            Chip8State::sub_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x6 {
            Chip8State::rsh_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0x7 {
            Chip8State::reverse_sub_reg
        } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0xe {
            Chip8State::lsh_reg
        } else if inst & 0xf000 == 0x9000 && inst & 0x000f == 0x0 {
            Chip8State::skip_regs_neq
        } else if inst & 0xf000 == 0xa000 {
            Chip8State::set_addr
        } else if inst & 0xf000 == 0xb000 {
            Chip8State::jmp_plus
        } else if inst & 0xf000 == 0xc000 {
            Chip8State::rand
        } else if inst & 0xf000 == 0xd000 {
            Chip8State::draw
        } else if inst & 0xf000 == 0xe000 && inst & 0x00ff == 0x9e {
            Chip8State::skip_if_pressed
        } else if inst & 0xf000 == 0xe000 && inst & 0x00ff == 0xa1 {
            Chip8State::skip_if_not_pressed
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x07 {
            Chip8State::get_delay_timer
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x0a {
            Chip8State::get_keypress
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x15 {
            Chip8State::set_delay_timer
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x18 {
            Chip8State::set_sound_timer
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x1e {
            Chip8State::add_to_addr
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x29 {
            Chip8State::set_addr_to_sprite_addr
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x33 {
            Chip8State::store_bcd
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x55 {
            Chip8State::reg_dump
        } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x65 {
            Chip8State::reg_load
        } else {
            panic!("bad instruction {:04x}", inst);
        }
    }

    // 0NNN
    pub fn call_rca1802_code_routine(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0);
        self.pc += 2;
    }

    // 00E0
    pub fn clear_display(&mut self, inst: u16) {
        assert_eq!(inst, 0x00e0);
        self.screen = [[false; 64]; 32];
        self.pc += 2;
    }

    // 00ee
    pub fn return_from_subroutine(&mut self, inst: u16) {
        assert_eq!(inst, 0x00ee);
        let addr = self.stack.pop();
        self.pc = addr;
    }

    // 1NNN
    pub fn jmp(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 1);
        let nnn = inst & 0x0fff;
        self.pc = nnn;
    }

    // 2NNN
    pub fn call(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 2);
        self.stack.push(self.pc + 2);
        let nnn = inst & 0x0fff;
        self.pc = nnn;
    }

    // 3XNN
    pub fn skip_eq(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 3);
        let nn = (inst & 0x00ff) as u8;
        let x = ((inst & 0x0f00) >> 8) as usize;
        if self.reg[x] == nn {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 4XNN
    pub fn skip_neq(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 4);
        let nn = (inst & 0x00ff) as u8;
        let x = ((inst & 0x0f00) >> 8) as usize;
        if self.reg[x] != nn {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 5XY0
    pub fn skip_regs_eq(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 5);
        assert_eq!(inst & 0x000f, 0);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        if self.reg[x] == self.reg[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 6XNN
    pub fn set_val(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 6);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let nn: u8 = (inst & 0x00ff) as u8;
        self.reg[x] = nn;
        self.pc += 2;
    }

    // 7XNN
    pub fn add_val(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 7);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let nn: u8 = (inst & 0x00ff) as u8;
        self.reg[x] = self.reg[x].wrapping_add(nn);
        self.pc += 2;
    }

    // 8XY0
    pub fn set_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 0);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] = self.reg[y];
        self.pc += 2;
    }

    // 8XY1
    pub fn or_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 1);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] |= self.reg[y];
        self.pc += 2;
    }

    // 8XY2
    pub fn and_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 2);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] &= self.reg[y];
        self.pc += 2;
    }

    // 8XY3
    pub fn xor_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 3);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] ^= self.reg[y];
        self.pc += 2;
    }

    // 8XY4
    pub fn add_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 4);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        let (result, carry) = self.reg[x].overflowing_add(self.reg[y]);
        self.reg[x] = result;
        self.reg[0xf] = carry as u8;
        self.pc += 2;
    }

    // 8XY5
    pub fn sub_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 5);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        let (result, borrow) = self.reg[x].overflowing_sub(self.reg[y]);
        self.reg[x] = result;
        self.reg[0xf] = !borrow as u8;
        self.pc += 2;
    }

    // 8XY6
    pub fn rsh_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 6);
        let x = ((inst & 0x0f00) >> 8) as usize;
        // So the order of operations is correct when performing operations
        // on the 0xf register.
        let tmp = self.reg[x] & 0x01;
        self.reg[x] >>= 1;
        self.reg[0xf] = tmp;
        self.pc += 2;
    }

    // 8XY7
    pub fn reverse_sub_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 7);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        let (result, borrow) = self.reg[y].overflowing_sub(self.reg[x]);
        self.reg[x] = result;
        self.reg[0xf] = !borrow as u8;
        self.pc += 2;
    }

    // 8XYE
    pub fn lsh_reg(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 8);
        assert_eq!(inst & 0x000f, 0xe);
        let x = ((inst & 0x0f00) >> 8) as usize;
        // See comment on 8XY6.
        let tmp = (self.reg[x] & 0x80) >> 7;
        self.reg[x] <<= 1;
        self.reg[0xf] = tmp;
        self.pc += 2;
    }

    // 9XY0
    pub fn skip_regs_neq(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 9);
        assert_eq!(inst & 0x000f, 0);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        if self.reg[x] != self.reg[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // ANNN
    pub fn set_addr(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xa);
        self.addr = inst & 0x0fff;
        self.pc += 2;
    }

    // BNNN
    pub fn jmp_plus(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xb);
        let nnn = inst & 0x0fff;
        self.pc = nnn + self.reg[0] as u16;
    }

    // CXNN
    pub fn rand(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xc);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let nn: u8 = (inst & 0xff) as u8;
        self.reg[x] = rand::thread_rng().gen();
        self.reg[x] &= nn;
        self.pc += 2;
    }

    // DXYN
    pub fn draw(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xd);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        let n: u8 = (inst & 0xf) as u8;

        let x = self.reg[x] as usize;
        let y = self.reg[y] as usize;
        let mut carry = 0;
        for i in 0..n {
            let byte = self.memory.read_t::<u8>(self.addr as usize +i as usize);
            for j in 0..8 {
                let y = (y + i as usize) % self.screen.len();
                let x = (x + j as usize) % self.screen[y].len();
                let bit = ((byte >> (7-j)) & 1) == 1;
                let before = self.screen[y][x];
                self.screen[y][x] ^= bit;
                if before && !self.screen[y][x] {
                    carry = 1;
                }
            }
        }
        self.reg[0xf] = carry;
        self.pc += 2;
    }

    // EX9E
    pub fn skip_if_pressed(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xe);
        assert_eq!(inst & 0x00ff, 0x9e);
        let x = ((inst & 0x0f00) >> 8) as usize;
        if self.key_pressed[self.reg[x] as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // EXA1
    pub fn skip_if_not_pressed(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xe);
        assert_eq!(inst & 0x00ff, 0xa1);
        let x = ((inst & 0x0f00) >> 8) as usize;
        if !self.key_pressed[self.reg[x] as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // FX07
    pub fn get_delay_timer(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 7);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.reg[x] = self.delay_timer;
        self.pc += 2;
    }

    // FX0A
    pub fn get_keypress(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x0a);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.keypress_halt = true;
        self.keypress_reg = x as u8;
        self.pc += 2;
    }

    // FX15
    pub fn set_delay_timer(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x15);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.delay_timer = self.reg[x];
        self.pc += 2;
    }

    // FX18
    pub fn set_sound_timer(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x18);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.sound_timer = self.reg[x];
        self.pc += 2;
    }

    // FX1E
    pub fn add_to_addr(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x1e);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.addr += self.reg[x] as u16;
        self.pc += 2;
    }

    // FX29
    pub fn set_addr_to_sprite_addr(&mut self, inst: u16) {
        println!("TODOTODOTODO");
        // TODO
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x29);
        let _x = ((inst & 0x0f00) >> 8) as usize;
        self.pc += 2;
    }

    // FX33
    pub fn store_bcd(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x33);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let mut number = self.reg[x];
        for i in (0..=2).rev() {
            let digit = number % 10;
            number /= 10;
            self.memory.write((self.addr+i) as usize, &digit.to_ne_bytes());
        }
        self.pc += 2;
    }

    // FX55
    pub fn reg_dump(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x55);
        let x = ((inst & 0x0f00) >> 8) as usize;
        for i in 0..x+1 {
            let to_write = self.reg[i].to_ne_bytes();
            self.memory.write(self.addr as usize + i, &to_write);
        }
        self.pc += 2;
    }

    // FX65
    pub fn reg_load(&mut self, inst: u16) {
        assert_eq!((inst & 0xf000) >> 12, 0xf);
        assert_eq!(inst & 0x00ff, 0x65);
        let x = ((inst & 0x0f00) >> 8) as usize;
        for i in 0..x+1 {
            let readed = self.memory.read(self.addr as usize + i, 1);
            let value = readed[0];
            self.reg[i] = value;
        }
        self.pc += 2;
    }
}
