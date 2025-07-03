use crate::instructions;
use crate::registers;

pub struct CPU {
    registers: registers::Registers,
    ime: bool,
    opcode: u8,
    cycles: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: registers::Registers::new(),
            ime: false,
            opcode: 0,
            cycles: 0,
        }
    }

    pub fn fetch_next(&mut self) -> u8 {}

    pub fn decode(opcode: u8, cb_opcode: bool) -> instructions::Instruction {}

    pub fn execute_next(&mut self) -> u64 {}
}
