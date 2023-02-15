use crate::stack::Stack;
use crate::memory::Memory;
use rand::Rng;

pub struct Chip8State {
    pc: u16,
    // V0 to VF
    reg: [u8; 16],
    // The I address register
    addr: u16,
    stack: Stack,
    memory: Memory,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8State {
    pub fn new() -> Chip8State {
        Chip8State {
            pc: 0,
            reg: [0; 16],
            addr: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    // 0NNN
    pub fn call_rca1802_code_routine(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0);
        // not necessary for most roms so gonna skip it
        self.pc += 2;
    }

    // 00E0
    pub fn clear_display(&mut self, inst: u16) {
        assert!(inst == 0x00e0);
        // TODO
        self.pc += 2;
    }

    // 00ee
    pub fn return_from_subroutine(&mut self, inst: u16) {
        assert!(inst == 0x00ee);
        let addr = self.stack.pop();
        self.pc = addr;
    }

    // 1NNN
    pub fn jmp(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 1);
        let nnn = inst & 0x0fff;
        self.pc = nnn;
    }

    // 2NNN
    pub fn call(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 2);
        self.stack.push(self.pc + 2);
        let nnn = inst & 0x0fff;
        self.pc = nnn;
    }

    // 3XNN
    pub fn skip_eq(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 3);
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
        assert!((inst & 0xf000) >> 12 == 4);
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
        assert!((inst & 0xf000) >> 12 == 5);
        assert!(inst & 0x000f == 0);
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
        assert!((inst & 0xf000) >> 12 == 6);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let nn: u8 = (inst & 0x00ff) as u8;
        self.reg[x] = nn;
        self.pc += 2;
    }

    // 7XNN
    pub fn add_val(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 7);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let nn: u8 = (inst & 0x00ff) as u8;
        self.reg[x] += nn;
        self.pc += 2;
    }

    // 8XY0
    pub fn set_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 0);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] += self.reg[y];
        self.pc += 2;
    }

    // 8XY1
    pub fn or_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 1);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] |= self.reg[y];
        self.pc += 2;
    }

    // 8XY2
    pub fn and_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 2);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] &= self.reg[y];
        self.pc += 2;
    }

    // 8XY3
    pub fn xor_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 3);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        self.reg[x] ^= self.reg[y];
        self.pc += 2;
    }

    // 8XY4
    pub fn add_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 4);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        let (result, carry) = self.reg[x].overflowing_add(self.reg[y]);
        self.reg[x] = result;
        self.reg[0xf] = carry as u8;
        self.pc += 2;
    }

    // 8XY5
    pub fn sub_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 5);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        let (result, borrow) = self.reg[x].overflowing_sub(self.reg[y]);
        self.reg[x] = result;
        self.reg[0xf] = borrow as u8;
        self.pc += 2;
    }

    // 8XY6
    pub fn rsh_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 6);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.reg[0xf] = self.reg[x] & 0x0001;
        self.reg[x] >>= 1;
        self.pc += 2;
    }

    // 8XY7
    pub fn reverse_sub_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 7);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let y = ((inst & 0x00f0) >> 4) as usize;
        let (result, borrow) = self.reg[y].overflowing_sub(self.reg[x]);
        self.reg[x] = result;
        self.reg[0xf] = borrow as u8;
        self.pc += 2;
    }

    // 8XYE
    pub fn lsh_reg(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 8);
        assert!(inst & 0x000f == 0xe);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.reg[0xf] = (self.reg[x] & 0x80) >> 7 as u8;
        self.reg[x] <<= 1;
        self.pc += 2;
    }

    // 9XY0
    pub fn skip_inst(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 9);
        assert!(inst & 0x000f == 0);
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
        assert!((inst & 0xf000) >> 12 == 0xa);
        self.addr = (inst & 0x0fff) as u16;
        self.pc += 2;
    }

    // BNNN
    pub fn jmp_plus(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xb);
        let nnn = inst & 0x0fff;
        self.pc = nnn + self.reg[0] as u16;
    }

    // CXNN
    pub fn rand(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xc);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let nn: u8 = (inst & 0xff) as u8;
        self.reg[x] = rand::thread_rng().gen();
        self.reg[x] &= nn;
        self.pc += 2;
    }

    // DXYN
    pub fn draw(&mut self, inst: u16) {
        // TODO
        assert!((inst & 0xf000) >> 12 == 0xd);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let x = ((inst & 0x00f0) >> 4) as usize;
        let n: u8 = (inst & 0xf) as u8;
        self.pc += 2;
    }

    // EX9E
    pub fn skip_if_pressed(&mut self, inst: u16) {
        // TODO
        assert!((inst & 0xf000) >> 12 == 0xe);
        assert!(inst & 0x00ff == 0x9e);
        let x = ((inst & 0x0f00) >> 8) as usize;
        if self.reg[x] == 0 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // EXA1
    pub fn skip_if_not_pressed(&mut self, inst: u16) {
        // TODO
        assert!((inst & 0xf000) >> 12 == 0xe);
        assert!(inst & 0x00ff == 0xa1);
        let x = ((inst & 0x0f00) >> 8) as usize;
        if self.reg[x] != 0 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // FX07
    pub fn get_delay_timer(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 07);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.reg[x] = self.delay_timer;
        self.pc += 2;
    }

    // FX0A
    pub fn get_keypress(&mut self, inst: u16) {
        // TODO
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x0a);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.reg[x] = 0;
        self.pc += 2;
    }

    // FX15
    pub fn set_delay_timer(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x15);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.delay_timer = self.reg[x];
        self.pc += 2;
    }

    // FX18
    pub fn set_sound_timer(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x18);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.sound_timer = self.reg[x];
        self.pc += 2;
    }

    // FX1E
    pub fn add_to_addr(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x1e);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.addr += self.reg[x] as u16;
        self.pc += 2;
    }

    // FX29
    pub fn set_addr_to_sprite_addr(&mut self, inst: u16) {
        // TODO
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x29);
        let x = ((inst & 0x0f00) >> 8) as usize;
        self.pc += 2;
    }

    // FX33
    pub fn store_bcd(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x33);
        let x = ((inst & 0x0f00) >> 8) as usize;
        let mut number = self.reg[x];
        for i in 0..3 {
            let digit = number % 10;
            number /= 10;
            self.memory.write((self.addr+i) as usize, &digit.to_ne_bytes());
        }
        self.pc += 2;
    }

    // FX55
    pub fn reg_dump(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x55);
        let x = ((inst & 0x0f00) >> 8) as usize;
        for i in 0..x+1 {
            let to_write = self.reg[i].to_ne_bytes();
            self.memory.write(self.addr as usize + i, &to_write);
        }
    }

    // FX65
    pub fn reg_load(&mut self, inst: u16) {
        assert!((inst & 0xf000) >> 12 == 0xf);
        assert!(inst & 0x00ff == 0x65);
        let x = ((inst & 0x0f00) >> 8) as usize;
        for i in 0..x+1 {
            let readed = self.memory.read(self.addr as usize + i, 1);
            let value = readed[0];
            self.reg[i] = value;
        }
    }
}
