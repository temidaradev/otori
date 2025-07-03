use crate::registers;

pub struct CPU {
    registers: registers::Registers,
    ime: bool,
    opcode: u8,
    cycled: u64,
}

impl CPU {}
