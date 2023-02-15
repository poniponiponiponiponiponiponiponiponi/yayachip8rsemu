pub struct Stack {
    sp: usize,
    stack: [u16; 16],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            sp: 0,
            stack: [0u16; 16],
        }
    }

    pub fn push(&mut self, val: u16) {
        self.stack[self.sp] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp]
    }
}
