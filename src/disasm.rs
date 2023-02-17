use core::fmt;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum InstructionType {
    CallRca1802CodeRoutine,
    ClearDisplay,
    ReturnFromSubroutine,
    Jmp,
    Call,
    SkipEq,
    SkipNeq,
    SkipRegsEq,
    SetVal,
    AddVal,
    SetReg,
    OrReg,
    AndReg,
    XorReg,
    AddReg,
    SubReg,
    RshReg,
    ReverseSubReg,
    LshReg,
    SkipRegsNeq,
    SetAddr,
    JmpPlus,
    Rand,
    Draw,
    SkipIfPressed,
    SkipIfNotPressed,
    GetDelayTimer,
    GetKeypress,
    SetDelayTimer,
    SetSoundTimer,
    AddToAddr,
    SetAddrToSpriteAddr,
    StoreBcd,
    RegDump,
    RegLoad,
    BadInstruction,
}

impl InstructionType {
    fn get_string(instruction_type: InstructionType) ->  &'static str {
        match instruction_type {
            Self::CallRca1802CodeRoutine => "call_rca1802_code_routine",
            Self::ClearDisplay => "clear_display",
            Self::ReturnFromSubroutine => "return",
            Self::Jmp => "jmp",
            Self::Call => "call",
            Self::SkipEq => "skip_eq",
            Self::SkipNeq => "skip_neq",
            Self::SkipRegsEq => "skip_eq",
            Self::SetVal => "set",
            Self::AddVal => "add",
            Self::SubReg => "sub",
            Self::SetReg => "set",
            Self::OrReg => "or",
            Self::AndReg => "and",
            Self::XorReg => "xor",
            Self::AddReg => "add",
            Self::RshReg => "rsh",
            Self::ReverseSubReg => "reverse_sub",
            Self::LshReg => "lsh",
            Self::SkipRegsNeq => "skip_neq",
            Self::SetAddr => "set",
            Self::JmpPlus => "jmp",
            Self::Rand => "rand",
            Self::Draw => "draw",
            Self::SkipIfPressed => "skip_if_pressed",
            Self::SkipIfNotPressed => "skip_if_not_pressed",
            Self::GetDelayTimer => "get_delay_timer",
            Self::GetKeypress => "get_keypress",
            Self::SetDelayTimer => "set_delay_timer",
            Self::SetSoundTimer => "set_sound_timer",
            Self::AddToAddr => "add",
            Self::SetAddrToSpriteAddr => "set_addr_to_sprite_addr",
            Self::StoreBcd => "store_bcd",
            Self::RegDump => "reg_dump",
            Self::RegLoad => "reg_load",
            Self::BadInstruction => "bad_instruction",
            _ => panic!("bad instruction type")
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub args: String,
}

impl Instruction {
    pub fn from(inst: u16) -> Instruction {
        find_instruction_func(inst)(inst)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}",
                 InstructionType::get_string(self.instruction_type),
                 self.args,
        )?;
        Ok(())
    }
}

fn find_instruction_func(inst: u16) -> fn(u16) -> Instruction {
    if inst == 0x00e0 {
        clear_display
    } else if inst == 0x00ee {
        return_from_subroutine
    } else if inst & 0xf000 == 0x0000 {
        call_rca1802_code_routine
    } else if inst & 0xf000 == 0x1000 {
        jmp
    } else if inst & 0xf000 == 0x2000 {
        call
    } else if inst & 0xf000 == 0x3000 {
        skip_eq
    } else if inst & 0xf000 == 0x4000 {
        skip_neq
    } else if inst & 0xf000 == 0x5000 && inst & 0x000f == 0 {
        skip_regs_eq
    } else if inst & 0xf000 == 0x6000 {
        set_val
    } else if inst & 0xf000 == 0x7000 {
        add_val
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0 {
        set_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 1 {
        or_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 2 {
        and_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 3 {
        xor_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 4 {
        add_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 5 {
        sub_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 6 {
        rsh_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 7 {
        reverse_sub_reg
    } else if inst & 0xf000 == 0x8000 && inst & 0x000f == 0xe {
        lsh_reg
    } else if inst & 0xf000 == 0x9000 && inst & 0x000f == 0 {
        skip_regs_neq
    } else if inst & 0xf000 == 0xa000 {
        set_addr
    } else if inst & 0xf000 == 0xb000 {
        jmp_plus
    } else if inst & 0xf000 == 0xc000 {
        rand
    } else if inst & 0xf000 == 0xd000 {
        draw
    } else if inst & 0xf000 == 0xe000 && inst & 0x00ff == 0xe9 {
        skip_if_pressed
    } else if inst & 0xf000 == 0xe000 && inst & 0x00ff == 0xa1 {
        skip_if_not_pressed
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x07 {
        get_delay_timer
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x0a {
        get_keypress
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x15 {
        set_delay_timer
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x18 {
        set_sound_timer
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x1e {
        add_to_addr
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x29 {
        set_addr_to_sprite_addr
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x33 {
        store_bcd
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x55 {
        reg_dump
    } else if inst & 0xf000 == 0xf000 && inst & 0x00ff == 0x65 {
        reg_load
    } else {
        bad_instruction
    }
}

// 0NNN
fn call_rca1802_code_routine(inst: u16) -> Instruction {
    assert!(inst & 0xf000 == 0x0000);
    Instruction {
        instruction_type: InstructionType::CallRca1802CodeRoutine,
        args: String::from(""),
    }
}

// 00E0
fn clear_display(inst: u16) -> Instruction {
    assert!(inst == 0x00e0);
    Instruction {
        instruction_type: InstructionType::ClearDisplay,
        args: String::from(""),
    }
}

// 00ee
fn return_from_subroutine(inst: u16) -> Instruction {
    assert!(inst == 0x00ee);
    Instruction {
        instruction_type: InstructionType::ReturnFromSubroutine,
        args: String::from(""),
    }
}

// 1NNN
fn jmp(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 1);
    let nnn = inst & 0x0fff;
    Instruction {
        instruction_type: InstructionType::Jmp,
        args: format!("{:04x}", nnn),
    }
}

// 2NNN
fn call(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 2);
    let nnn = inst & 0x0fff;
    Instruction {
        instruction_type: InstructionType::Call,
        args: format!("{:04x}", nnn),
    }
}

// 3XNN
fn skip_eq(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 3);
    let nn = (inst & 0x00ff) as u8;
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::SkipEq,
        args: format!("reg[{}] {:02x}", x, nn),
    }
}

// 4XNN
fn skip_neq(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 4);
    let nn = (inst & 0x00ff) as u8;
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::SkipNeq,
        args: format!("reg[{}] {:02x}", x, nn),
    }
}

// 5XY0
fn skip_regs_eq(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 5);
    assert!(inst & 0x000f == 0);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::SkipRegsEq,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 6XNN
fn set_val(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 6);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let nn: u8 = (inst & 0x00ff) as u8;
    Instruction {
        instruction_type: InstructionType::SetVal,
        args: format!("reg[{}] {:02x}", x, nn),
    }
}

// 7XNN
fn add_val(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 7);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let nn: u8 = (inst & 0x00ff) as u8;
    Instruction {
        instruction_type: InstructionType::AddVal,
        args: format!("reg[{}] {:02x}", x, nn),
    }
}

// 8XY0
fn set_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 0);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::SetReg,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 8XY1
fn or_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 1);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::OrReg,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 8XY2
fn and_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 2);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::AndReg,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 8XY3
fn xor_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 3);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::XorReg,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 8XY4
fn add_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 4);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::AddReg,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 8XY5
fn sub_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 5);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::SubReg,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 8XY6
fn rsh_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 6);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::RshReg,
        args: format!("reg[{}]", x),
    }
}

// 8XY7
fn reverse_sub_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 7);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::ReverseSubReg,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// 8XYE
fn lsh_reg(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 8);
    assert!(inst & 0x000f == 0xe);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::LshReg,
        args: format!("reg[{}]", x),
    }
}

// 9XY0
fn skip_regs_neq(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 9);
    assert!(inst & 0x000f == 0);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    Instruction {
        instruction_type: InstructionType::SkipRegsEq,
        args: format!("reg[{}] reg[{}]", x, y),
    }
}

// ANNN
fn set_addr(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xa);
    let nnn = (inst & 0x0fff) as u16;
    Instruction {
        instruction_type: InstructionType::SetAddr,
        args: format!("I {:04x}", nnn),
    }
}

// BNNN
fn jmp_plus(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xb);
    let nnn = inst & 0x0fff;
    Instruction {
        instruction_type: InstructionType::JmpPlus,
        args: format!("{:04x} + reg[0]", nnn),
    }
}

// CXNN
fn rand(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xc);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let nn: u8 = (inst & 0xff) as u8;
    Instruction {
        instruction_type: InstructionType::Rand,
        args: format!("reg[{}] {:02x}", x, nn),
    }
}

// DXYN
fn draw(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xd);
    let x = ((inst & 0x0f00) >> 8) as usize;
    let y = ((inst & 0x00f0) >> 4) as usize;
    let n: u8 = (inst & 0xf) as u8;
    Instruction {
        instruction_type: InstructionType::Draw,
        args: format!("reg[{}] reg[{}] {:02x}", x, y, n),
    }
}

// EX9E
fn skip_if_pressed(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xe);
    assert!(inst & 0x00ff == 0x9e);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::SkipIfPressed,
        args: format!("reg[{}]", x),
    }
}

// EXA1
fn skip_if_not_pressed(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xe);
    assert!(inst & 0x00ff == 0xa1);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::SkipIfNotPressed,
        args: format!("reg[{}]", x),
    }
}

// FX07
fn get_delay_timer(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 07);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::GetDelayTimer,
        args: format!("reg[{}]", x),
    }
}

// FX0A
fn get_keypress(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x0a);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::GetKeypress,
        args: format!("reg[{}]", x),
    }
}

// FX15
fn set_delay_timer(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x15);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::GetDelayTimer,
        args: format!("reg[{}]", x),
    }
}

// FX18
fn set_sound_timer(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x18);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::SetSoundTimer,
        args: format!("reg[{}]", x),
    }
}

// FX1E
fn add_to_addr(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x1e);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::AddToAddr,
        args: format!("reg[{}]", x),
    }
}

// FX29
fn set_addr_to_sprite_addr(inst: u16) -> Instruction {
    // TODO
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x29);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::SetAddrToSpriteAddr,
        args: format!("reg[{}]", x),
    }
}

// FX33
fn store_bcd(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x33);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::StoreBcd,
        args: format!("reg[{}]", x),
    }
}

// FX55
fn reg_dump(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x55);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::RegDump,
        args: format!("reg[{}]", x),
    }
}

// FX65
fn reg_load(inst: u16) -> Instruction {
    assert!((inst & 0xf000) >> 12 == 0xf);
    assert!(inst & 0x00ff == 0x65);
    let x = ((inst & 0x0f00) >> 8) as usize;
    Instruction {
        instruction_type: InstructionType::RegLoad,
        args: format!("reg[{}]", x),
    }
}

// everything else
fn bad_instruction(_inst: u16) -> Instruction {
    Instruction {
        instruction_type: InstructionType::BadInstruction,
        args: format!(""),
    }
}
