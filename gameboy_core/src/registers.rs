pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE, // Initial stack pointer value
            pc: 0x0100, // Initial program counter value
        }
    }

    pub fn reset(&mut self) {
        *self = Registers::new();
    }
}
