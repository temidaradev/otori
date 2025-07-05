pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

const FLAG_ZERO: u8 = 0b10000000;
const FLAG_NEGATIVE: u8 = 0b01000000;
const FLAG_HALF_CARRY: u8 = 0b00100000;
const FLAG_CARRY: u8 = 0b00010000;

macro_rules! get_set {
    ($reg:ident, $get_name:ident, $set_name:ident, $size:ty) => {
        pub fn $get_name(&self) -> $size {
            self.$reg
        }

        pub fn $set_name(&mut self, val: $size) {
            self.$reg = val;
        }
    };
}

macro_rules! get_set_dual {
    ($reg1:ident, $reg2:ident, $get_name:ident, $set_name:ident) => {
        pub fn $get_name(&self) -> u16 {
            (self.$reg1 as u16) << 8 | self.$reg2 as u16
        }

        pub fn $set_name(&mut self, val: u16) {
            self.$reg1 = (val >> 8) as u8;
            self.$reg2 = val as u8;
        }
    };
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    get_set!(a, get_a, set_a, u8);
    get_set!(b, get_b, set_b, u8);
    get_set!(c, get_c, set_c, u8);
    get_set!(d, get_d, set_d, u8);
    get_set!(e, get_e, set_e, u8);
    get_set!(h, get_h, set_h, u8);
    get_set!(l, get_l, set_l, u8);
    get_set!(sp, get_sp, set_sp, u16);
    get_set!(pc, get_pc, set_pc, u16);

    get_set_dual!(b, c, get_bc, set_bc);
    get_set_dual!(d, e, get_de, set_de);
    get_set_dual!(a, f, get_af, set_af);

    pub fn get_zero_flag(&self) -> bool {
        (self.f & FLAG_ZERO) != 0
    }

    pub fn set_zero_flag(&mut self, val: bool) {
        if val {
            self.f |= FLAG_ZERO;
        } else {
            self.f &= !FLAG_ZERO;
        }
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.f & FLAG_CARRY) != 0
    }

    pub fn set_carry_flag(&mut self, val: bool) {
        if val {
            self.f |= FLAG_CARRY;
        } else {
            self.f &= !FLAG_CARRY;
        }
    }

    pub fn get_half_carry_flag(&self) -> bool {
        (self.f & FLAG_HALF_CARRY) != 0
    }

    pub fn set_half_carry_flag(&mut self, val: bool) {
        if val {
            self.f |= FLAG_HALF_CARRY;
        } else {
            self.f &= !FLAG_HALF_CARRY;
        }
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.f & FLAG_NEGATIVE) != 0
    }

    pub fn set_negative_flag(&mut self, val: bool) {
        if val {
            self.f |= FLAG_NEGATIVE;
        } else {
            self.f &= !FLAG_NEGATIVE;
        }
    }

    pub fn get_f(&self) -> u8 {
        self.f
    }

    pub fn set_f(&mut self, val: u8) {
        self.f = val & 0xF0
    }

    pub fn reset(&mut self) {
        *self = Registers::new();
    }
}
