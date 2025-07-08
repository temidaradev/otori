use rand::random;
use std::fmt;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

#[derive(Debug)]
pub enum EmuError {
    StackOverflow,
    StackUnderflow,
    RamOutOfBounds,
    InvalidKey,
    ProgramTooLarge,
    UnimplementedOpcode(u16),
}

impl fmt::Display for EmuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmuError::StackOverflow => write!(f, "Stack overflow"),
            EmuError::StackUnderflow => write!(f, "Stack underflow"),
            EmuError::RamOutOfBounds => write!(f, "RAM access out of bounds"),
            EmuError::InvalidKey => write!(f, "Invalid key pressed"),
            EmuError::ProgramTooLarge => write!(f, "Program size exceeds available RAM"),
            EmuError::UnimplementedOpcode(op) => write!(f, "Unimplemented opcode: {:04X}", op),
        }
    }
}

impl std::error::Error for EmuError {}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) -> Result<(), EmuError> {
        let op = self.fetch()?;

        self.execute(op)?;
        Ok(())
    }

    pub fn timer_tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }
            self.st -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) -> Result<(), EmuError> {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        if end > RAM_SIZE {
            return Err(EmuError::ProgramTooLarge);
        }
        self.ram[start..end].copy_from_slice(data);
        Ok(())
    }

    fn execute(&mut self, op: u16) -> Result<(), EmuError> {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => Ok(()),
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
                Ok(())
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop()?;
                self.pc = ret_addr;
                Ok(())
            }
            // JMP NNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
                Ok(())
            }
            // CALL NNN
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc)?;
                self.pc = nnn;
                Ok(())
            }
            // SKIP VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
                Ok(())
            }
            // SKIP VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
                Ok(())
            }
            // SKIP VX == VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
                Ok(())
            }
            // VX = NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
                Ok(())
            }
            // VX += NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
                Ok(())
            }
            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
                Ok(())
            }
            // VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
                Ok(())
            }
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
                Ok(())
            }
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
                Ok(())
            }
            // VX += VY
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
                Ok(())
            }
            // VX -= VY
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
                Ok(())
            }
            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
                Ok(())
            }
            // VX = VY - VX
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
                Ok(())
            }
            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
                Ok(())
            }
            // SKIP VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
                Ok(())
            }
            // I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
                Ok(())
            }
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
                Ok(())
            }
            // VX = rand() & NN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
                Ok(())
            }
            // DRAW
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                let num_rows = digit4;

                let mut flipped = false;

                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;

                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
                Ok(())
            }
            // SKIP KEY PRESS
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                if (vx as usize) >= NUM_KEYS {
                    return Err(EmuError::InvalidKey);
                }
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
                Ok(())
            }
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                if (vx as usize) >= NUM_KEYS {
                    return Err(EmuError::InvalidKey);
                }
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
                Ok(())
            }
            // VX = DT
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
                Ok(())
            }
            // WAIT KEY
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // Redo opcode
                    self.pc -= 2;
                }
                Ok(())
            }
            // DT = VX
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
                Ok(())
            }
            // ST = VX
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v_reg[x];
                Ok(())
            }
            // I += VX
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
                Ok(())
            }
            // I = FONT
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;
                Ok(())
            }
            // BCD
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                let hundreds = (vx / 100.0).floor() as u8;

                let tens = ((vx / 10.0) % 10.0).floor() as u8;

                let ones = (vx % 10.0) as u8;
                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
                Ok(())
            }
            // STORE V0 - VX
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
                Ok(())
            }
            // LOAD V0 - VX
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
                Ok(())
            }
            (_, _, _, _) => Err(EmuError::UnimplementedOpcode(op)),
        }
    }

    fn fetch(&mut self) -> Result<u16, EmuError> {
        if self.pc as usize + 1 >= RAM_SIZE {
            return Err(EmuError::RamOutOfBounds);
        }
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        Ok(op)
    }

    fn push(&mut self, value: u16) -> Result<(), EmuError> {
        if self.sp as usize >= self.stack.len() {
            return Err(EmuError::StackOverflow);
        }
        self.stack[self.sp as usize] = value;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<u16, EmuError> {
        if self.sp == 0 {
            return Err(EmuError::StackUnderflow);
        }
        self.sp -= 1;
        Ok(self.stack[self.sp as usize])
    }
}
