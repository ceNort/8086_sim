use std::fs;
use std::collections::HashMap;
use std::fmt;

// #[derive(Debug)]
// enum Register {
//     AL = 0b000,
//     CL = 0b001,
//     DL = 0b010,
//     BL = 0b011,
//     AH = 0b100,
//     CH = 0b101,
//     DH = 0b110,
//     BH = 0b111
// }

// #[derive(Debug)]
// enum WideRegister {
//     AX = 0b000,
//     CX = 0b001,
//     DX = 0b010,
//     BX = 0b011,
//     SP = 0b100,
//     BP = 0b101,
//     SI = 0b110,
//     DI = 0b111
// }

// impl fmt::Display for Register {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl fmt::Display for WideRegister {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }


// enum Mode {
//     Mem = 0b00,
//     Mem8 = 0b01,
//     Mem16 = 0b10,
//     Reg = 0b11,
// }

#[derive(Debug)]
struct Instruction {
    opcode: u8, // Instruction
    d: bool, // Reg is Destination 
    w: bool, // Wide (16 bits)
    mode: u8, // Mode
    reg: u8,
    r_m: u8,
}

impl Instruction {
    fn new(inst: &[u8]) -> Instruction {
        let first_byte = inst[0];
        let second_byte = inst[1];

        Instruction {
                opcode: (first_byte >> 2)  & 0b111111,
                d: ((first_byte >> 1) & 0b1) != 0,
                w: (first_byte & 0b1) != 0,
                mode: (second_byte >> 6) & 0b11,
                reg: (second_byte >> 3) & 0b111,
                r_m: second_byte & 0b111,
        }
    }

    fn source_dest(&self) -> (&str, &str) {
        let reg_hash = match self.w {
            false => HashMap::from([
                (0b000 as u8, "AL"),
                (0b001 as u8, "CL"),
                (0b010 as u8, "DL"),
                (0b011 as u8, "BL"),
                (0b100 as u8, "AH"),
                (0b101 as u8, "CH"),
                (0b110 as u8, "DH"),
                (0b111 as u8, "BH")
            ]),
            true => HashMap::from([
                (0b000 as u8, "AX"),
                (0b001 as u8, "CX"),
                (0b010 as u8, "DX"),
                (0b011 as u8, "BX"),
                (0b100 as u8, "SP"),
                (0b101 as u8, "BP"),
                (0b110 as u8, "SI"),
                (0b111 as u8, "DI")
            ])
        };

        let (source, dest) = match self.d {
            false => (self.reg, self.r_m),
            true => (self.r_m, self.reg)
        };

        (reg_hash.get(&source).unwrap(), reg_hash.get(&dest).unwrap())
    }
}

fn main() {
    let opcodes = HashMap::from([
        (0b100010 as u8, "MOV"),
        (0b110001 as u8, "MOV"),
    ]);

    let buffer = fs::read("data/listing_0038_many_register_mov")
        .expect("Failed to read file!");

    let mut output = String::from("bits 16\n");

    for instruction in buffer.chunks(2) {
        let inst = Instruction::new(instruction);

        let opcode = opcodes.get(&inst.opcode).unwrap();
        let (source, dest) = inst.source_dest();
        
        let inst_string = format!("{} {}, {}\n", opcode, dest, source);

        output.push_str(&inst_string);
    }

    fs::write("data/output.asm", output).expect("Unable to write file");
}
