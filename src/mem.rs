#[allow(non_camel_case_types)]

use std::fmt::{self, Formatter};

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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

#[derive(Debug)]
pub struct MemLoc {
    name: String,
    value: u8,
}

impl MemLoc {
    pub fn new(name: String) -> Self {
        MemLoc {
            name,
            value: 0,
        }
    }

    pub fn write(&mut self, val: u8) {
        self.value = val;
    }

    pub fn read(self) -> u8 { self.value }
}

#[derive(Debug)]
pub struct Memory {
    ax: MemLoc,
    cx: MemLoc,
    dx: MemLoc,
    bx: MemLoc,
    sp: MemLoc,
    bp: MemLoc,
    si: MemLoc,
    di: MemLoc,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ax: MemLoc::new(String::from("AX")),
            cx: MemLoc::new(String::from("CX")),
            dx: MemLoc::new(String::from("DX")),
            bx: MemLoc::new(String::from("BX")),
            sp: MemLoc::new(String::from("SP")),
            bp: MemLoc::new(String::from("BP")),
            si: MemLoc::new(String::from("SI")),
            di: MemLoc::new(String::from("DI")),
        }
    }

    pub fn get_loc(&mut self, loc: &str) -> Option<&mut MemLoc> {
        match loc {
            "AX" => Some(&mut self.ax),
            "CX" => Some(&mut self.cx),
            "DX" => Some(&mut self.dx),
            "BX" => Some(&mut self.bx),
            "SP" => Some(&mut self.sp),
            "BP" => Some(&mut self.bp),
            "SI" => Some(&mut self.si),
            "DI" => Some(&mut self.di),
            _ => None,
        }
    }
}