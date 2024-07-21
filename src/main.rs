#![allow(dead_code, unused_variables, non_camel_case_types)]

mod instruction;
mod mem;

use instruction::*;
use std::fs;
use std::convert::From;
use std::env;
use std::process::exit;
use mem::Memory;

// TODO:
//   - Finish implementing commented out opcodes?


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("You need to provide a filepath!");
        exit(1);
    }

    let filepath = args.last().expect("No filename provided");

    let mut flags: Vec<&str> = Vec::new();
    if args.len() > 2 {
        for idx in 1..(args.len() - 1) {
            flags.push(&args[idx]);
        }
    }

    let debug = flags.contains(&"debug");
    let bindump = flags.contains(&"bindump");
    let to_file = flags.contains(&"file");

    // If output, create dir if it doesn't exist
    if debug || bindump || to_file {
        fs::create_dir_all("output").expect("Failed to create output directory");
    }

    let buffer = fs::read(filepath)
        .expect("Failed to read file!");

    if bindump {
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

        fs::write("output/orig_hex.txt", hex_str).expect("Failed to write orig hex file.");
        fs::write("output/orig_bytes.txt", bin_str).expect("Failed to write orig bin file.");
    }

    // Initialize Memory
    let mut main_mem = Memory::new();

    let mut debug_output = String::new(); // For debug file

    let instructions: Vec<Instruction> = read_buffer_into_instructions(buffer, bindump, &mut debug_output);

    if debug {
        fs::write("output/debug_output.txt", &debug_output).expect("Failed to write debug output file.");
    }

    let mut asm_output = String::from("bits 16\n");

    for inst in instructions {
        let mem_loc_string = inst.dest.clone();

        // Write to output
        let inst_string = format!("{} {}, {} ; ", inst.str_val, &mem_loc_string, inst.source);

        // Write current mem val to output
        let pre_mem_string = format!("{}:0x{:02x}", &mem_loc_string, main_mem.clone().read_loc(&mem_loc_string));

        asm_output.push_str(&inst_string);
        asm_output.push_str(&pre_mem_string);

        // Execute any MOV instructions
        match inst.opcode {
            Opcode::MovImmToReg => inst.execute(&mut main_mem),
            Opcode::MovRmToReg => inst.execute(&mut main_mem),
            _ => todo!("Only MovImmToReg and MovRmToReg implemented"),
        }

        // Write new mem val to output
        let post_mem_string = format!("->{}:0x{:02x}\n", &mem_loc_string, main_mem.clone().read_loc(&mem_loc_string));
        asm_output.push_str(&post_mem_string);
    }

    // Add final state of all changed registers to output
    asm_output.push_str("\n\n; Final Registers:\n");
    for reg in Memory::loc_list() {
        let reg_val = &main_mem.clone().read_loc(&reg);
        let reg_str = format!(";    {}: 0x{:04x} ({})\n", reg, reg_val, reg_val);
        asm_output.push_str(&reg_str);
    }

    println!("{asm_output}");

    if to_file {
        fs::write("output/output.asm", asm_output).expect("Unable to write ASM file");
    }
}

fn read_buffer_into_instructions(buffer: Vec<u8>, debug: bool, debug_output: &mut String) -> Vec<Instruction> {
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
            let instruction = Instruction::new(opcode, &buffer[index..index+offset]);

            if debug {
                for byte in &buffer[index..index + offset] {
                    debug_output.push_str(&format!("{:08b} ({:02X})\n", byte, byte));
                }

                if (index + offset) < buffer.len() {
                    debug_output.push_str(&format!("PEEK NEXT BYTE: {:08b} ({:02X})\n", &buffer[index + offset], &buffer[index + offset]));
                }

                debug_output.push_str(&format!("{} {}, {}\n\n", instruction.str_val, instruction.dest, instruction.source));
            }

            instructions.push(instruction);
            index += offset;
        } else {
            // Break the cycle because something fucked up
            println!("Something really fucked up");
            index = buffer.len() + 1;
        }
    }

    instructions
}