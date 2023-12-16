#![allow(dead_code, unused_variables)]
use std::fs;
use std::convert::From;

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
enum OpType {
    ADD = 0b000,
    SUB = 0b101,
    CMP = 0b111,
    UNIMPL
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
enum Opcode {
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
enum Mode {
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

#[derive(Debug)]
struct Instruction {
    raw_bin: String,
    opcode: Opcode,
    d: bool, // Reg is Destination 
    w: bool, // Wide (16 bits)
    s: Option<bool>, // Combined with W for ImmReg Add/Sub/Cmp
    mode: Option<Mode>,
    reg: u8,
    r_m: Option<u8>,
    disp_lo: Option<u8>,
    disp_hi: Option<u8>,
    data: Option<u16>,
    dest: String,
    source: String,
    str_val: String
}

impl Instruction {
    fn get_reg_str(reg: u8, w: bool) -> String {
        let reg_str = match reg {
            0b000 => if w { "AX" } else { "AL" },
            0b001 => if w { "CX" } else { "CL" },
            0b010 => if w { "DX" } else { "DL" },
            0b011 => if w { "BX" } else { "BL" },
            0b100 => if w { "SP" } else { "AH" },
            0b101 => if w { "BP" } else { "CH" },
            0b110 => if w { "SI" } else { "DH" },
            0b111 => if w { "DI" } else { "BH" },
            _ => "FAIL"
        };

        String::from(reg_str)
    }

    fn get_mem_str(r_m: &u8) -> String {
        let mem_str = match r_m {
            0b000 => "BX + SI",
            0b001 => "BX + DI",
            0b010 => "BP + SI",
            0b011 => "BP + DI",
            0b100 => "SI",
            0b101 => "DI",
            0b110 => "BP",
            0b111 => "BX",
            _ => "FAIL"
        };

        String::from(mem_str)
    }

    fn new(opcode: Opcode, full_inst: &[u8]) -> Instruction {
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
                let reg =  (second_byte >> 3) & 0b111;
                let r_m =  Some(second_byte & 0b111);
                let data = None;
                
                let (disp_lo, disp_hi, dest, source) = match Mode::from(mode_bits) {
                    Mode::Mem => {
                        match r_m.unwrap() {
                            0b110 => {
                                let disp_lo = Some(full_inst[2]);
                                let disp_hi = Some(full_inst[3]);
                                let rm = format!("[{}]",Instruction::get_mem_str(&r_m.unwrap()));
                                let rg = Instruction::get_reg_str(reg, w);

                                let (dest, source) = match d {
                                    false => (rm, rg),
                                    true => (rg, rm)
                                };                                

                                (disp_lo, disp_hi, dest, source)
                            },
                            _ => {
                                let disp_lo = None;
                                let disp_hi = None;                                
                                let rm =  format!("[{}]",Instruction::get_mem_str(&r_m.unwrap()));
                                let rg = Instruction::get_reg_str(reg, w);

                                let (dest, source) = match d {
                                    false => (rm, rg),
                                    true => (rg, rm)
                                };

                                (disp_lo, disp_hi, dest, source)
                            }
                        }
                    },
                    Mode::Mem8 => {
                        let disp_lo: Option<u8> = Some(full_inst[2]);
                        let disp_hi: Option<u8> =  None;
                        let rm = format!("[{} + {}]", Instruction::get_mem_str(&r_m.unwrap()), disp_lo.unwrap());
                        let rg = Instruction::get_reg_str(reg, w);

                        let (dest, source) = match d {
                            false => (rm, rg),
                            true => (rg, rm)
                        };

                        (disp_lo, disp_hi, dest, source)
                    },
                    Mode::Mem16 => {
                        let disp_lo = Some(full_inst[2]); 
                        let disp_hi = Some(full_inst[3]);
                        let full_disp = u16::from(disp_hi.unwrap()) << 8 | u16::from(disp_lo.unwrap());
                        let rm = format!("[{} + {}]", Instruction::get_mem_str(&r_m.unwrap()), full_disp);
                        let rg = Instruction::get_reg_str(reg, w);

                        let (dest, source) = match d {
                            false => (rm, rg),
                            true => (rg, rm)
                        };

                        (disp_lo, disp_hi, dest, source)
                    },
                    Mode::Reg => {
                        let disp_lo = None;
                        let disp_hi = None;
                        let rm = Instruction::get_reg_str(r_m.unwrap(), w); // TODO: Is this right?
                        let rg = Instruction::get_reg_str(reg, w);

                        let (dest, source) = match d {
                            false => (rm, rg),
                            true => (rg, rm)
                        };

                        (disp_lo, disp_hi, dest, source)
                    }
                };

                let str_val = match opcode {
                    Opcode::MovRmToReg => String::from("MOV"),
                    Opcode::AddRmAndReg => String::from("ADD"),
                    Opcode::SubRmAndReg => String::from("SUB"),
                    Opcode::CmpRmAndReg => String::from("CMP"),
                    _ => String::from("UNIMPL")
                };

                (d, w, s, mode, reg, r_m, disp_lo, disp_hi, data, dest, source, str_val)
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
                    reg = first_byte & 0b111;
                } else {
                    reg = 0b000;
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

                let dest = Instruction::get_reg_str(reg, w);
                let source: String = match w {
                    true => format!("{}", data.unwrap()),
                    false => format!("{}", data.unwrap() as i8)
                };

                let str_val = match opcode {
                    Opcode::MovImmToReg => String::from("MOV"),
                    Opcode::AddImmToAcc => String::from("ADD"),
                    Opcode::SubImmFromAcc => String::from("SUB"),
                    Opcode::CmpImmToAcc => String::from("CMP"),
                    _ => String::from("UNIMPL")
                };                

                (d, w, s, mode, reg, r_m, disp_lo, disp_hi, data, dest, source, str_val)
            },
            Opcode::ImmToRm => {
                let inst_len = full_inst.len();

                let second_byte = full_inst[1];
                let mode_bits = (second_byte >> 6) & 0b11;

                let d = false;
                let w = (first_byte & 0b1) != 0;
                let s = ((first_byte >> 1) & 0b1) != 0;
                let mode =  Some(Mode::from(mode_bits));
                let op_type =  OpType::from((second_byte >> 3) & 0b111);
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
                        
                        let dest = match r_m.unwrap() {
                            0b110 => format!("[{}]", u16::from(disp_hi.unwrap()) << 8 | u16::from(disp_lo.unwrap())),
                            _ => format!("[{}]",Instruction::get_mem_str(&r_m.unwrap()))
                        };

                        (disp_lo, disp_hi, dest)
                    },
                    Mode::Mem8 => {
                        let disp_lo: Option<u8> = Some(full_inst[2]);
                        let disp_hi: Option<u8> =  None;
                        let dest = format!("[{} + {}]", Instruction::get_mem_str(&r_m.unwrap()), disp_lo.unwrap());
                        
                        (disp_lo, disp_hi, dest)
                    },
                    Mode::Mem16 => {
                        let disp_lo = Some(full_inst[2]); 
                        let disp_hi = Some(full_inst[3]);
                        let full_disp = u16::from(disp_hi.unwrap()) << 8 | u16::from(disp_lo.unwrap());
                        let dest = format!("[{} + {}]", Instruction::get_mem_str(&r_m.unwrap()), full_disp);
                        
                        (disp_lo, disp_hi, dest)
                    },
                    Mode::Reg => {
                        let disp_lo = None;
                        let disp_hi = None;
                        let dest = Instruction::get_reg_str(r_m.unwrap(), w);

                        (disp_lo, disp_hi, dest)
                    }
                };

                let mut str_val = match op_type {
                    OpType::ADD => String::from("ADD"),
                    OpType::SUB => String::from("SUB"),
                    OpType::CMP => String::from("CMP"),
                    _ => String::from("UNIMPL")
                };

                match mode.unwrap() {
                    Mode::Reg => {},
                    _ => {
                        match w {
                            true => str_val.push_str(" WORD"),
                            false => str_val.push_str(" BYTE"),
                        }
                    }
                }

                (d, w, Some(s), mode, op_type as u8, r_m, disp_lo, disp_hi, data, dest, source, str_val) // Will op_type fuck this up?
            },
            Opcode::JmpEqual | Opcode::JmpLess| Opcode::JmpLessOrEqual | Opcode::JmpBelow | Opcode::JmpBelowOrEqual |
            Opcode::JmpParity | Opcode::JmpOverflow | Opcode::JmpSign | Opcode::JmpNotEqual | Opcode::JmpNotLess | Opcode::JmpNotLessOrEqual |
            Opcode::JmpNotBelow | Opcode::JmpNotBelowOrEqual | Opcode::JmpNotParity | Opcode::JmpNotOverflow | Opcode::JmpOnNotSign |
            Opcode::Loop | Opcode::LoopZero | Opcode::LoopNotZero | Opcode::JmpCXZero => {
                let d = false;
                let w = false;
                let s = None;
                let mode = None;
                let reg = 0;
                let r_m = None;
                let disp_lo = None;
                let disp_hi = None;
                let data = None;
                let dest = String::from("label");

                let source = format!("{}", full_inst[1] as i8);

                let str_val = match opcode {
                    Opcode::JmpEqual           => String::from("JE"),
                    Opcode::JmpLess            => String::from("JL"), 
                    Opcode::JmpLessOrEqual     => String::from("JLE"),
                    Opcode::JmpBelow           => String::from("JB"),
                    Opcode::JmpBelowOrEqual    => String::from("JBE"),
                    Opcode::JmpParity          => String::from("JP"),
                    Opcode::JmpOverflow        => String::from("JO"),
                    Opcode::JmpSign            => String::from("JS"),
                    Opcode::JmpNotEqual        => String::from("JNZ"),
                    Opcode::JmpNotLess         => String::from("JNL"),
                    Opcode::JmpNotLessOrEqual  => String::from("JNLE"),
                    Opcode::JmpNotBelow        => String::from("JNB"),
                    Opcode::JmpNotBelowOrEqual => String::from("JNBE"),
                    Opcode::JmpNotParity       => String::from("JNP"),
                    Opcode::JmpNotOverflow     => String::from("JNO"),
                    Opcode::JmpOnNotSign       => String::from("JNS"),
                    Opcode::Loop               => String::from("LOOP"),
                    Opcode::LoopZero           => String::from("LOOPZ"),
                    Opcode::LoopNotZero        => String::from("LOOPNZ"),
                    Opcode::JmpCXZero          => String::from("JCXZ"),
                    _ => String::from("UNIMPL")
                };

                
                (d, w, s, mode, reg, r_m, disp_lo, disp_hi, data, dest, source, str_val)
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

fn main() {
    let buffer = fs::read("data/listing_0041_add_sub_cmp_jnz")
        .expect("Failed to read file!");

    // Write binary to file for easy reading
    let mut hex_str = String::new();
    let mut bin_str = String::new();
    let mut idx = 1;
    while idx <= buffer.len() {
        let byte_val = buffer[idx - 1];
        hex_str.push_str(&format!("{:02X} ", byte_val));
        bin_str.push_str(&format!("{:08b} ", byte_val));
        if idx % 8 == 0 {
            bin_str.push_str("\n");
        }
        if idx % 16 == 0 {
            hex_str.push_str("\n");
        }
        idx += 1;
    }

    fs::write("data/orig_hex.txt", hex_str).expect("Failed to write orig hex file.");
    fs::write("data/orig_bytes.txt", bin_str).expect("Failed to write orig bin file.");

    let mut debug_output = String::new(); // For debug file

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut index = 0;

    while index < buffer.len() {
        let first_byte = buffer[index];

        let opcode;
        
        let offset = match (first_byte >> 4) & 0b1111 { // Get first four bits
            0b1000 => {
                match (first_byte >> 2) & 0b111111 {
                    0b100010 => {
                        // MovRmToReg
                        // 100010 D W | MOD REG R/M 
                        // if MOD == 01 (DISP-LO)
                        // if MOD == 10 (DISP-HI)
                        opcode = Opcode::MovRmToReg;
                        let second_byte = buffer[index + 1];
                        let mode = (second_byte >> 6) & 0b11;
                        let r_m = second_byte & 0b111;
                        match mode {
                            0b00 => {
                                match r_m {
                                    0b110 => 4,
                                    _ => 2
                                }
                            },
                            0b01 => 3,
                            0b10 => 4,
                            0b11 => 2,
                            _ => 0
                        }
                    },
                    0b100000 => {                        
                        // ImmToRm,
                        // Could be ADD, SUB, or CMP - doesn't matter here, only length of instruction
                        opcode = Opcode::ImmToRm;
                        let second_byte = buffer[index + 1];
                        let mode = (second_byte >> 6) & 0b11;
                        let r_m = second_byte & 0b111;
                        let s_w = first_byte & 0b11;
                        let disp_count = match mode {
                            0b00 => {
                                match r_m {
                                    0b110 => 2,
                                    _ => 0
                                }
                            },
                            0b01 => 1,
                            0b10 => 2,
                            0b11 => 0,
                            _ => 0
                        };

                        // Automatically Inst/mod-000-rm/data
                        // Maybe disp-lo/disp-hi before data
                        // if s_w = 01 add extra data
                        match s_w {
                            0b00 | 0b11 => disp_count + 3,
                            0b01 => disp_count + 4,
                            _ => 0,
                        }
                    },
                    _ => {
                        opcode = Opcode::Unimpl;
                        0
                    }
                }
            },
            0b1011 => {
                // 1011 W REG | DATA
                // if W = 1 (DATA)
                opcode = Opcode::MovImmToReg;
                let w = (first_byte >> 3) & 0b1;

                match w {
                    0b0 => 2,
                    0b1 => 3,
                    _ => 0
                }
            },
            0b0000 => {
                // ADD: Could be AddRmAndReg or AddImmToAcc
                let first_bits = (first_byte >> 2) & 0b111111;

                match first_bits {
                    0b000000 => {
                        opcode = Opcode::AddRmAndReg;
                        // if MOD == 01 (DISP-LO)
                        // if MOD == 10 (DISP-HI)
                        let second_byte = buffer[index + 1];
                        let mode = (second_byte >> 6) & 0b11;
                        let r_m = second_byte & 0b111;
                        match mode {
                            0b00 => {
                                match r_m {
                                    0b110 => 4,
                                    _ => 2
                                }
                            },
                            0b01 => 3,
                            0b10 => 4,
                            0b11 => 2,
                            _ => 0
                        }
                    },
                    0b000001 => {
                        opcode = Opcode::AddImmToAcc;
                        let w = first_byte & 0b1;

                        match w {
                            0b0 => 2,
                            0b1 => 3,
                            _ => 0
                        }
                    },
                    _ => {
                        opcode = Opcode::Unimpl;
                        0
                    }
                }
            },
            0b0010 => {
                // SUB: Could be SubRmAndReg or SubImmFromAcc
                let first_bits = (first_byte >> 2) & 0b111111;

                match first_bits {
                    0b001010 => {
                        opcode = Opcode::SubRmAndReg;
                        // if MOD == 01 (DISP-LO)
                        // if MOD == 10 (DISP-HI)
                        let second_byte = buffer[index + 1];
                        let mode = (second_byte >> 6) & 0b11;
                        let r_m = second_byte & 0b111;
                        match mode {
                            0b00 => {
                                match r_m {
                                    0b110 => 4,
                                    _ => 2
                                }
                            },
                            0b01 => 3,
                            0b10 => 4,
                            0b11 => 2,
                            _ => 0
                        }
                    },
                    0b001011 => {
                        opcode = Opcode::SubImmFromAcc;
                        let w = first_byte & 0b1;

                        match w {
                            0b0 => 2,
                            0b1 => 3,
                            _ => 0
                        }
                    },
                    _ => {
                        opcode = Opcode::Unimpl;
                        0
                    }
                }
            },
            0b0011 => {
                // CMP: Could be CmpRmAndReg or CmpImmToAcc
                let first_bits = (first_byte >> 2) & 0b111111;

                match first_bits {
                    0b001110 => {
                        opcode = Opcode::CmpRmAndReg;
                        // if MOD == 01 (DISP-LO)
                        // if MOD == 10 (DISP-HI)
                        let second_byte = buffer[index + 1];
                        let mode = (second_byte >> 6) & 0b11;
                        let r_m = second_byte & 0b111;
                        match mode {
                            0b00 => {
                                match r_m {
                                    0b110 => 4,
                                    _ => 2
                                }
                            },
                            0b01 => 3,
                            0b10 => 4,
                            0b11 => 2,
                            _ => 0
                        }
                    },
                    0b001111 => {
                        opcode = Opcode::CmpImmToAcc;
                        let w = first_byte & 0b1;

                        match w {
                            0b0 => 2,
                            0b1 => 3,
                            _ => 0
                        }
                    },
                    _ => {
                        opcode = Opcode::Unimpl;
                        0
                    },
                }
            },
            0b0111 | 0b1110 => {
                // Jump or Loop
               opcode = Opcode::from(first_byte);
               2 // Always 2
            },
            _ => {
                opcode = Opcode::Unimpl;
                println!("SOMETHING FUCKED UP");
                0
            }
        };

        if offset > 0 {
            for byte in &buffer[index..index+offset] {
                debug_output.push_str(&format!("{:08b} ({:02X})\n", byte, byte));
            }
 
            if (index + offset) < buffer.len() {
                debug_output.push_str(&format!("PEEK NEXT BYTE: {:08b} ({:02X})\n", &buffer[index+offset], &buffer[index+offset]));
            }

            let instruction = Instruction::new(opcode, &buffer[index..index+offset]);

            debug_output.push_str(&format!("{} {}, {}\n\n", instruction.str_val, instruction.dest, instruction.source));

            instructions.push(instruction);
            index += offset;
        } else {
            // Break the cycle because something fucked up
            println!("Something really fucked up");
            index = buffer.len() + 1;
        }
    }

    fs::write("data/debug_output.txt", &debug_output).expect("Failed to write debug output file.");

    let mut asm_output = String::from("bits 16\n");

    for inst in instructions {
        let inst_string = format!("{} {}, {}\n", inst.str_val, inst.dest, inst.source);

        asm_output.push_str(&inst_string);
    }        

    fs::write("data/output.asm", asm_output).expect("Unable to write ASM file");
}
