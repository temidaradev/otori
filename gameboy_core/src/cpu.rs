use crate::instructions::Instruction;
use crate::ram::RAM;
use crate::registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub memory: RAM,
    ime: bool,
    opcode: u8,
    cycles: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            memory: RAM::new(),
            ime: false,
            opcode: 0,
            cycles: 0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory.write(address, value);
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low
    }

    pub fn read_word_pc(&mut self) -> u16 {
        let pc = self.registers.get_pc();
        let word = self.read_word(pc);
        self.registers.set_pc(pc + 2);
        word
    }

    pub fn fetch_next(&mut self) -> u8 {
        let pc = self.registers.get_pc();
        let byte = self.read_byte(pc);
        self.registers.set_pc(pc + 1);
        byte
    }

    pub fn decode(opcode: u8, cb_opcode: bool) -> Option<Instruction> {
        Instruction::from_byte(opcode)
    }

    pub fn execute_next(&mut self) -> u64 {
        let opcode = self.fetch_next();
        if let Some(instruction) = Self::decode(opcode, false) {
            instruction.execute(self) as u64
        } else {
            panic!("Unknown opcode: 0x{:02X}", opcode);
        }
    }
}
