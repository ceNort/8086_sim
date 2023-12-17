use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OpType {
    ADD = 0b000,
    SUB = 0b101,
    CMP = 0b111,
    UNIMPL
}

impl fmt::Display for OpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for OpType {
    fn from(value: u8) -> Self {
        match value {
            0b000 => OpType::ADD,
            0b101 => OpType::SUB,
            0b111 => OpType::CMP,
            _ => OpType::UNIMPL
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Opcode {
    MovRmToReg         = 0b100010,
    // MovImmToRm      = 0b1100011,
    MovImmToReg        = 0b1011,
    // MovMemToAcc     = 0b1010000,
    // MovAccToMem     = 0b1010001,
    AddRmAndReg        = 0b000000,
    AddImmToAcc        = 0b0000010,
    SubRmAndReg        = 0b001010,
    SubImmFromAcc      = 0b0010110,
    CmpRmAndReg        = 0b001110,
    CmpImmToAcc        = 0b0011110,
    ImmToRm            = 0b100000,
    JmpEqual           = 0b01110100,
    JmpLess            = 0b01111100,
    JmpLessOrEqual     = 0b01111110,
    JmpBelow           = 0b01110010,
    JmpBelowOrEqual    = 0b01110110,
    JmpParity          = 0b01111010,
    JmpOverflow        = 0b01110000,
    JmpSign            = 0b01111000,
    JmpNotEqual        = 0b01110101,
    JmpNotLess         = 0b01111101,
    JmpNotLessOrEqual  = 0b01111111,
    JmpNotBelow        = 0b01110011,
    JmpNotBelowOrEqual = 0b01110111,
    JmpNotParity       = 0b01111011,
    JmpNotOverflow     = 0b01110001,
    JmpOnNotSign       = 0b01111001,
    Loop               = 0b11100010,
    LoopZero           = 0b11100001,
    LoopNotZero        = 0b11100000,
    JmpCXZero          = 0b11100011,
    Unimpl             = 0b11111111,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ImmToRm handled separately
        match self {
            Self::MovImmToReg        => write!(f, "{}", "MOV"),
            Self::MovRmToReg         => write!(f, "{}", "MOV"),
            Self::AddImmToAcc        => write!(f, "{}", "ADD"),
            Self::AddRmAndReg        => write!(f, "{}", "ADD"),
            Self::SubImmFromAcc      => write!(f, "{}", "SUB"),
            Self::SubRmAndReg        => write!(f, "{}", "SUB"),
            Self::CmpImmToAcc        => write!(f, "{}", "CMP"),
            Self::CmpRmAndReg        => write!(f, "{}", "CMP"),
            Self::JmpEqual           => write!(f, "{}", "JE"),
            Self::JmpLess            => write!(f, "{}", "JL"),
            Self::JmpLessOrEqual     => write!(f, "{}", "JLE"),
            Self::JmpBelow           => write!(f, "{}", "JB"),
            Self::JmpBelowOrEqual    => write!(f, "{}", "JBE"),
            Self::JmpParity          => write!(f, "{}", "JP"),
            Self::JmpOverflow        => write!(f, "{}", "JO"),
            Self::JmpSign            => write!(f, "{}", "JS"),
            Self::JmpNotEqual        => write!(f, "{}", "JNZ"),
            Self::JmpNotLess         => write!(f, "{}", "JNL"),
            Self::JmpNotLessOrEqual  => write!(f, "{}", "JNLE"),
            Self::JmpNotBelow        => write!(f, "{}", "JNB"),
            Self::JmpNotBelowOrEqual => write!(f, "{}", "JNBE"),
            Self::JmpNotParity       => write!(f, "{}", "JNP"),
            Self::JmpNotOverflow     => write!(f, "{}", "JNO"),
            Self::JmpOnNotSign       => write!(f, "{}", "JNS"),
            Self::Loop               => write!(f, "{}", "LOOP"),
            Self::LoopZero           => write!(f, "{}", "LOOPZ"),
            Self::LoopNotZero        => write!(f, "{}", "LOOPNZ"),
            Self::JmpCXZero          => write!(f, "{}", "JCXZ"),
            _ => write!(f, "{}", "UNIMPL")
        }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0b100010 => Opcode::MovRmToReg,
            // 0b1100011 => Opcode::MovImmToRm,
            0b1101 => Opcode::MovImmToReg,
            // 0b1010000 => Opcode::MovMemToAcc,
            // 0b1010001 => Opcode::MovAccToMem,
            0b000000 => Opcode::AddRmAndReg,
            0b0000010 => Opcode::AddImmToAcc,
            0b001010 => Opcode::SubRmAndReg,
            0b0010110 => Opcode::SubImmFromAcc,
            0b001110 => Opcode::CmpRmAndReg,
            0b0011110 => Opcode::CmpImmToAcc,
            0b100000 => Opcode::ImmToRm,
            0b01110100 => Opcode::JmpEqual,
            0b01111100 => Opcode::JmpLess,
            0b01111110 => Opcode::JmpLessOrEqual,
            0b01110010 => Opcode::JmpBelow,
            0b01110110 => Opcode::JmpBelowOrEqual,
            0b01111010 => Opcode::JmpParity,
            0b01110000 => Opcode::JmpOverflow,
            0b01111000 => Opcode::JmpSign,
            0b01110101 => Opcode::JmpNotEqual,
            0b01111101 => Opcode::JmpNotLess,
            0b01111111 => Opcode::JmpNotLessOrEqual,
            0b01110011 => Opcode::JmpNotBelow,
            0b01110111 => Opcode::JmpNotBelowOrEqual,
            0b01111011 => Opcode::JmpNotParity,
            0b01110001 => Opcode::JmpNotOverflow,
            0b01111001 => Opcode::JmpOnNotSign,
            0b11100010 => Opcode::Loop,
            0b11100001 => Opcode::LoopZero,
            0b11100000 => Opcode::LoopNotZero,
            0b11100011 => Opcode::JmpCXZero,
            _ => Opcode::Unimpl
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Reg { // TODO: Does this need to be pub?
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
pub enum Mode { // TODO: Does this need to be pub?
    Mem = 0b00,
    Mem8 = 0b01,
    Mem16 = 0b10,
    Reg = 0b11,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Mode::Mem,
            0b01 => Mode::Mem8,
            0b10 => Mode::Mem16,
            0b11 => Mode::Reg,
            _ => todo!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EffectiveAddress { // TODO: does this need to be pub?
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

#[derive(Debug)]
pub struct Instruction { // TODO: Do all of these need to be pub? Maybe none and accessors?
    pub raw_bin: String,
    pub opcode: Opcode,
    pub d: bool, // Reg is Destination
    pub w: bool, // Wide (16 bits)
    pub s: Option<bool>, // Combined with W for ImmReg Add/Sub/Cmp
    pub mode: Option<Mode>,
    pub reg: Reg,
    pub r_m: Option<u8>,
    pub disp_lo: Option<u8>,
    pub disp_hi: Option<u8>,
    pub data: Option<u16>,
    pub dest: String,
    pub source: String,
    pub str_val: String
}

impl Instruction {
    pub fn new(opcode: Opcode, full_inst: &[u8]) -> Instruction {
        let mut raw_bin = String::new();
        for inst in full_inst {
            raw_bin.push_str(&format!("{:08b}", inst));
        }
        let first_byte = full_inst[0];

        let (d, w, s, mode, reg, r_m, disp_lo, disp_hi, data, dest, source, str_val) = match opcode {
            Opcode::MovRmToReg | Opcode::AddRmAndReg | Opcode::SubRmAndReg | Opcode::CmpRmAndReg => {
                let second_byte = full_inst[1];
                let mode_bits = (second_byte >> 6) & 0b11;

                let d = ((first_byte >> 1) & 0b1) != 0;
                let w = (first_byte & 0b1) != 0;
                let s = None;
                let mode =  Some(Mode::from(mode_bits));
                let reg =  Reg::from((second_byte >> 3 & 0b111) << 1 | u8::from(w));
                let r_m =  Some(second_byte & 0b111);
                let data = None;

                let (disp_lo, disp_hi, dest, source) = match Mode::from(mode_bits) {
                    Mode::Mem => {
                        match r_m.unwrap() {
                            0b110 => {
                                let disp_lo = Some(full_inst[2]);
                                let disp_hi = Some(full_inst[3]);
                                let rm = EffectiveAddress::from(r_m.unwrap());

                                let (dest, source) = match d {
                                    false => (rm.to_string(), reg.to_string()),
                                    true => (reg.to_string(), rm.to_string())
                                };

                                (disp_lo, disp_hi, dest, source)
                            },
                            _ => {
                                let disp_lo = None;
                                let disp_hi = None;
                                let rm = EffectiveAddress::from(r_m.unwrap());

                                let (dest, source) = match d {
                                    false => (rm.to_string(), reg.to_string()),
                                    true => (reg.to_string(), rm.to_string())
                                };

                                (disp_lo, disp_hi, dest, source)
                            }
                        }
                    },
                    Mode::Mem8 => {
                        let disp_lo: Option<u8> = Some(full_inst[2]);
                        let disp_hi: Option<u8> =  None;
                        let rm = EffectiveAddress::from(r_m.unwrap());
                        let dest_str = format!("[{} + {}]", rm.to_string(), disp_lo.unwrap());

                        let (dest, source) = match d {
                            false => (dest_str, reg.to_string()),
                            true => (reg.to_string(), dest_str)
                        };

                        (disp_lo, disp_hi, dest, source)
                    },
                    Mode::Mem16 => {
                        let disp_lo = Some(full_inst[2]);
                        let disp_hi = Some(full_inst[3]);
                        let full_disp = u16::from(disp_hi.unwrap()) << 8 | u16::from(disp_lo.unwrap());
                        let rm = EffectiveAddress::from(r_m.unwrap());
                        let dest_str = format!("[{} + {}]", rm.to_string(), full_disp);

                        let (dest, source) = match d {
                            false => (dest_str, reg.to_string()),
                            true => (reg.to_string(), dest_str)
                        };

                        (disp_lo, disp_hi, dest, source)
                    },
                    Mode::Reg => {
                        let disp_lo = None;
                        let disp_hi = None;
                        let rm = Reg::from(r_m.unwrap() << 1 | u8::from(w));

                        let (dest, source) = match d {
                            false => (rm.to_string(), reg.to_string()),
                            true => (reg.to_string(), rm.to_string())
                        };

                        (disp_lo, disp_hi, dest, source)
                    }
                };

                (d, w, s, mode, reg, r_m, disp_lo, disp_hi, data, dest, source, opcode.to_string())
            },
            Opcode::MovImmToReg | Opcode::AddImmToAcc | Opcode::SubImmFromAcc | Opcode::CmpImmToAcc => {
                let d = false;

                let w = match opcode {
                    Opcode::MovImmToReg => ((first_byte >> 3) & 0b1) != 0,
                    Opcode::AddImmToAcc | Opcode::SubImmFromAcc | Opcode::CmpImmToAcc => (first_byte & 0b1) != 0,
                    _ => false
                };

                let s = None;
                let mode = None;
                let r_m = None;
                let disp_lo = None;
                let disp_hi = None;

                let reg;
                if opcode == Opcode::MovRmToReg {
                    reg = Reg::from((first_byte & 0b111) << 1 | u8::from(w));
                } else {
                    reg = Reg::from(0b000 << 1 | u8::from(w));
                }

                let data = match w {
                    true => {
                        let full_data = u16::from(full_inst[2]) << 8 | u16::from(full_inst[1]);
                        Some(full_data)
                    },
                    false => {
                        Some(u16::from(full_inst[1]))
                    }
                };

                let dest = reg.to_string();
                let source: String = match w {
                    true => format!("{}", data.unwrap()),
                    false => format!("{}", data.unwrap() as i8)
                };

                (d, w, s, mode, reg, r_m, disp_lo, disp_hi, data, dest, source, opcode.to_string())
            },
            Opcode::ImmToRm => {
                let inst_len = full_inst.len();

                let second_byte = full_inst[1];
                let mode_bits = (second_byte >> 6) & 0b11;

                let d = false;
                let w = (first_byte & 0b1) != 0;
                let s = ((first_byte >> 1) & 0b1) != 0;
                let mode =  Some(Mode::from(mode_bits));
                let op_type =  OpType::from(second_byte >> 3 & 0b111);
                let reg = Reg::from((second_byte >> 3 & 0b111) << 1 | u8::from(w));
                let r_m =  Some(second_byte & 0b111);

                let data = match (s, w) {
                    (false, true) => {
                        let full_data = u16::from(full_inst[inst_len - 1]) << 8 | u16::from(full_inst[inst_len - 2]);
                        Some(full_data)
                    },
                    (false, false) | (true, true) | (true, false) => {
                        Some(u16::from(full_inst[inst_len - 1]))
                    }
                };

                let source = format!("{}", data.unwrap());

                let (disp_lo, disp_hi, dest) = match Mode::from(mode_bits) {
                    Mode::Mem => {
                        let (disp_lo, disp_hi) = match r_m.unwrap() {
                            0b110 => {
                                (Some(full_inst[2]), Some(full_inst[3]))
                            },
                            _ => {
                                (None, None)
                            }
                        };

                        let rm = EffectiveAddress::from(r_m.unwrap());

                        let dest = match r_m.unwrap() {
                            0b110 => format!("[{}]", u16::from(disp_hi.unwrap()) << 8 | u16::from(disp_lo.unwrap())),
                            _ => format!("[{}]", rm.to_string()),
                        };

                        (disp_lo, disp_hi, dest)
                    },
                    Mode::Mem8 => {
                        let disp_lo: Option<u8> = Some(full_inst[2]);
                        let disp_hi: Option<u8> =  None;
                        let rm = EffectiveAddress::from(r_m.unwrap());
                        let dest = format!("[{} + {}]", rm.to_string(), disp_lo.unwrap());

                        (disp_lo, disp_hi, dest)
                    },
                    Mode::Mem16 => {
                        let disp_lo = Some(full_inst[2]);
                        let disp_hi = Some(full_inst[3]);
                        let full_disp = u16::from(disp_hi.unwrap()) << 8 | u16::from(disp_lo.unwrap());
                        let rm = EffectiveAddress::from(r_m.unwrap());
                        let dest = format!("[{} + {}]", rm.to_string(), full_disp);

                        (disp_lo, disp_hi, dest)
                    },
                    Mode::Reg => {
                        let disp_lo = None;
                        let disp_hi = None;
                        let rm_reg = Reg::from(r_m.unwrap() << 1 | u8::from(w));
                        let dest = rm_reg.to_string();

                        (disp_lo, disp_hi, dest)
                    }
                };

                let mut str_val = op_type.to_string();

                match mode.unwrap() {
                    Mode::Reg => {},
                    _ => {
                        match w {
                            true => str_val.push_str(" WORD"),
                            false => str_val.push_str(" BYTE"),
                        }
                    }
                }
                (d, w, Some(s), mode, reg, r_m, disp_lo, disp_hi, data, dest, source, str_val)
            },
            Opcode::JmpEqual | Opcode::JmpLess| Opcode::JmpLessOrEqual | Opcode::JmpBelow | Opcode::JmpBelowOrEqual |
            Opcode::JmpParity | Opcode::JmpOverflow | Opcode::JmpSign | Opcode::JmpNotEqual | Opcode::JmpNotLess | Opcode::JmpNotLessOrEqual |
            Opcode::JmpNotBelow | Opcode::JmpNotBelowOrEqual | Opcode::JmpNotParity | Opcode::JmpNotOverflow | Opcode::JmpOnNotSign |
            Opcode::Loop | Opcode::LoopZero | Opcode::LoopNotZero | Opcode::JmpCXZero => {
                let d = false;
                let w = false;
                let s = None;
                let mode = None;
                let reg = Reg::UNIMPL; // NOTE: If there are issues, it may be here
                let r_m = None;
                let disp_lo = None;
                let disp_hi = None;
                let data = None;
                let dest = String::from("label");

                let source = format!("{}", full_inst[1] as i8);

                (d, w, s, mode, reg, r_m, disp_lo, disp_hi, data, dest, source, opcode.to_string())
            },
            Opcode::Unimpl => {
                panic!("YOU SHOULDN'T SEE THIS");
                // (0, 0, None, None, 0b000, 0b000, None, None, None)
            }
        };


        Instruction {
            raw_bin,
            opcode,
            d,
            w,
            s,
            mode,
            reg,
            r_m,
            disp_lo,
            disp_hi,
            data,
            dest,
            source,
            str_val
        }
    }
}