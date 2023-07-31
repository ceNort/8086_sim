#![allow(dead_code, unused_variables)]
use std::fs;
use std::fmt;
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Opcode {
    RmReg = 0b100010,
    // IMM_RM = 0b1100011,
    ImmReg = 0b1011,
    // MEM_ACC = 0b1010000,
    // ACC_MEM = 0b1010001    
    Unimpl
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0b100010 => Opcode::RmReg,
            // IMM_RM => IMM_RM,
            0b1011 => Opcode::ImmReg,
            // MEM_ACC => MEM_ACC,
            // ACC_MEM => ACC_MEM,
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

// TODO: Work on this!
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Can this be used to write "MOV"?
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    d: bool, // Reg is Destination 
    w: bool, // Wide (16 bits)
    mode: Option<Mode>,
    reg: u8,
    r_m: Option<u8>,
    disp_lo: Option<u8>,
    disp_hi: Option<u8>,
    data: Option<u16>,
    dest: String,
    source: String,
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
        let first_byte = full_inst[0];

        let (d, w, mode, reg, r_m, disp_lo, disp_hi, data, dest, source) = match opcode {
            Opcode::RmReg => {
                let second_byte = full_inst[1];
                let mode_bits = (second_byte >> 6) & 0b11;

                let d = ((first_byte >> 1) & 0b1) != 0;
                let w = (first_byte & 0b1) != 0;
                let mode =  Some(Mode::from(mode_bits));
                let reg =  (second_byte >> 3) & 0b111;
                let r_m =  Some(second_byte & 0b111);
                let data = None;

                for i in 0..full_inst.len() {
                    println!("Byte {}: {:b}", i, full_inst[i]);
                }
                
                println!("Mode: {:?}, R_M: {:?}", mode, r_m);
                
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

                (d, w, mode, reg, r_m, disp_lo, disp_hi, data, dest, source)
            },
            Opcode::ImmReg => {
                let d = false;
                let w = ((first_byte >> 3) & 0b1) != 0;
                let mode = None;
                let reg = first_byte & 0b111;
                let r_m = None;
                let disp_lo = None;
                let disp_hi = None;

                for i in 0..full_inst.len() {
                    println!("Byte {}: {:b}", i, full_inst[i]);
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
                let source = format!("{}", data.unwrap());

                (d, w, mode, reg, r_m, disp_lo, disp_hi, data, dest, source)
            },
            Opcode::Unimpl => {
                panic!("YOU SHOULDN'T SEE THIS");
                // (0, 0, None, 0b000, 0b000, None, None, None)
            }
        };

        Instruction {
            opcode,
            d,
            w,
            mode,
            reg,
            r_m,
            disp_lo,
            disp_hi,
            data,
            dest,
            source
        }
    }
}

fn main() {
    let opcodes = HashMap::from([
        (Opcode::ImmReg, "MOV"),
        (Opcode::RmReg, "MOV"),
        (Opcode::Unimpl, "UNIMPL"),
    ]);

    let buffer = fs::read("data/listing_0039_more_movs")
        .expect("Failed to read file!");

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut index = 0;

    while index < buffer.len() {
        let first_byte = buffer[index];

        let opcode;
        
        let offset = match (first_byte >> 4) & 0b1111 { // Get first four bits
            0b1000 => {
                // 100010 D W | MOD REG R/M 
                // if MOD == 01 (DISP-LO)
                // if MOD == 10 (DISP-HI)
                opcode = Opcode::RmReg;
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
            0b1011 => {
                // 1011 W REG | DATA
                // if W = 1 (DATA)
                opcode = Opcode::ImmReg;
                let w = (first_byte >> 3) & 0b1;

                match w {
                    0b0 => 2,
                    0b1 => 3,
                    _ => 0
                }
            },
            _ => {
                opcode = Opcode::Unimpl;
                println!("SOMETHING FUCKED UP");
                0
            }
        };

        if offset > 0 {
            let instruction = Instruction::new(opcode, &buffer[index..index+offset]);

            let opcode = opcodes.get(&instruction.opcode).unwrap();

            println!("{} {}, {}\n", opcode, instruction.dest, instruction.source);
            
            instructions.push(instruction);
            index += offset;
        } else {
            // Break the cycle because something fucked up
            println!("Something really fucked up");
            index = buffer.len() + 1;
        }
    }

    let mut output = String::from("bits 16\n");

    for inst in instructions {
        let opcode = opcodes.get(&inst.opcode).unwrap();

        let inst_string = format!("{} {}, {}\n", opcode, inst.dest, inst.source);

        output.push_str(&inst_string);
    }        

    fs::write("data/output.asm", output).expect("Unable to write file");
}
