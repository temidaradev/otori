use crate::registers;

pub struct cpu {
    registers: registers::Registers,
    ime: bool,
    opcode: u8,
    cycled: u64,
}
