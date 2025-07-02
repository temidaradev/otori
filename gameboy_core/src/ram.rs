pub struct ram {
    memory: [u8; 65536],
}

impl ram {
    pub fn new() -> Self {
        ram { memory: [0; 65536] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
    }
}
