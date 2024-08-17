pub struct ProgramCounter {
    pub pos: u8,
    jumped: bool,
}

impl ProgramCounter {
    #[inline]
    pub fn new() -> Self {
        Self {
            pos: 0,
            jumped: false,
        }
    }
}

impl ProgramCounter {
    #[inline]
    pub fn next(&mut self) -> u8 {
        if self.jumped {
            self.jumped = false;
        } else {
            self.pos += 1;
        }
        self.pos
    }
    #[inline]
    pub fn jump(&mut self, pos: u8) {
        self.pos = pos;
        self.jumped = true;
    }
}