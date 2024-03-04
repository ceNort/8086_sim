#[allow(non_camel_case_types)]

use std::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Reg {
    // REG = Instruction.reg && Instruction.w
    AL = 0b0000,
    AX = 0b0001,
    CX = 0b0011,
    CL = 0b0010,
    DX = 0b0101,
    DL = 0b0100,
    BX = 0b0111,
    BL = 0b0110,
    SP = 0b1001,
    AH = 0b1000,
    BP = 0b1011,
    CH = 0b1010,
    SI = 0b1101,
    DH = 0b1100,
    DI = 0b1111,
    BH = 0b1110,
    UNIMPL = 0b111111,
}

impl From<u8> for Reg {
    fn from(value: u8) -> Self{
        match value {
            0b0000 => Self::AL,
            0b0001 => Self::AX,
            0b0011 => Self::CX,
            0b0010 => Self::CL,
            0b0101 => Self::DX,
            0b0100 => Self::DL,
            0b0111 => Self::BX,
            0b0110 => Self::BL,
            0b1001 => Self::SP,
            0b1000 => Self::AH,
            0b1011 => Self::BP,
            0b1010 => Self::CH,
            0b1101 => Self::SI,
            0b1100 => Self::DH,
            0b1111 => Self::DI,
            0b1110 => Self::BH,
            _ => Self::UNIMPL
        }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EffectiveAddress {
BX_SI = 0b000,
    BX_DI = 0b001,
    BP_SI = 0b010,
    BP_DI = 0b011,
    SI = 0b100,
    DI = 0b101,
    BP = 0b110,
    BX = 0b111,
    UNIMPL = 0b11111111,
}

impl From<u8> for EffectiveAddress {
    fn from(value: u8) -> Self{
        match value {
            0b000 => Self::BX_SI,
            0b001 => Self::BX_DI,
            0b010 => Self::BP_SI,
            0b011 => Self::BP_DI,
            0b100 => Self::SI,
            0b101 => Self::DI,
            0b110 => Self::BP,
            0b111 => Self::BX,
            _ => Self::UNIMPL
        }
    }
}

impl fmt::Display for EffectiveAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ImmToRm handled separately
        match self {
            Self::BX_SI => write!(f, "{}", "BX + SI"),
            Self::BX_DI => write!(f, "{}", "BX + DI"),
            Self::BP_SI => write!(f, "{}", "BP + SI"),
            Self::BP_DI => write!(f, "{}", "BP + DI"),
            Self::SI => write!(f, "{}", "SI"),
            Self::DI => write!(f, "{}", "DI"),
            Self::BP => write!(f, "{}", "BP"),
            Self::BX => write!(f, "{}", "BX"),
            _ => write!(f, "{}", "UNIMPL")
        }
    }
}

pub struct Register {
    name: Reg,
    value: Option<u32>,
}

impl Register {
    pub fn new(name: Reg) -> Self {
        Register {
            name,
            value: None,
        }
    }

    pub fn write(&mut self, val: u32) {
        self.value = Some(val);
    }
}

pub struct Memory {
    al: Register,
    cl: Register,
    dl: Register,
    bl: Register,
    ah: Register,
    ch: Register,
    dh: Register,
    bh: Register,
    sp: Register,
    bp: Register,
    si: Register,
    di: Register,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            al: Register::new(Reg::AL),
            cl: Register::new(Reg::CL),
            dl: Register::new(Reg::DL),
            bl: Register::new(Reg::BL),
            ah: Register::new(Reg::AH),
            ch: Register::new(Reg::CH),
            dh: Register::new(Reg::DH),
            bh: Register::new(Reg::BH),
            sp: Register::new(Reg::SP),
            bp: Register::new(Reg::BP),
            si: Register::new(Reg::SI),
            di: Register::new(Reg::DI),
        }
    }

    pub fn ax(self) -> Register {
        // TODO: Left off here, I have no idea if this is right but my brain hurts tonight
        let val = (&self.ah.value.unwrap() << 4) | &self.al.value.unwrap();
        Register {
            name: Reg::AX,
            value: Some(val),
        }
    }
}