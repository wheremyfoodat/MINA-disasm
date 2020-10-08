const ARITHMETIC_IMMEDIATE_OPCODES: &[&str] = &[
    "addi", "multi", "divi", "remi", "slti", "sltiu", "nop", "pcaddi"
];

const ARITHMETIC_REGISTER_OPCODES: &[&str] = &[
    "add", "mult", "div", "rem", "slt", "sltu", "sub", "pcadd"
];

const CONDITION_CODES: &[&str] = &[
    "eq", "lo", "ls", "lt", "le"
];

const LOGICAL_IMMEDIATE_OPCODES: &[&str] = &[
    "andi", "ori", "xori", "nandi"
];

const LOGICAL_REGISTER_OPCODES: &[&str] = &[
    "and", "or", "xor", "nand", "popcnt", "clo", "plo"
];

pub fn disassemble_arithmetic (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let src1 = (instr >> 20) & 0xF;
    let src2 = (instr >> 16) & 0xF; // For I-type opcodes, this is a shift amount, NOT src2
    let dest = (instr >> 12) & 0xF;

    if opcode <= 0b111 {// Opcodes 0 to 7 are I-type and use 12-bit immediates embedded inside the instruction
        let mut imm = instr & 0xFFF; // Fetch 12 bit immediate
        imm = ((imm as i32) << 20 >> 20) as u32; // Sign extend it
        imm <<= src2; // Shift the immediate by the shift amount specified in src2

        if opcode == 0b110 {
            "nop".to_string()
        }

        else if opcode != 0b111 {
            format!("{} r{}, r{}, #0x{:08X}", ARITHMETIC_IMMEDIATE_OPCODES[opcode as usize], dest, src1, imm)
        }

        else { // PCADDI needs special treatment
            format!("{} #0x{:08X}", ARITHMETIC_IMMEDIATE_OPCODES[opcode as usize], imm)
        }
    }

    else {
        if opcode != 0b1111 { // PCADD needs special treatment
            format!("{} r{}, r{}, r{}", ARITHMETIC_REGISTER_OPCODES[opcode as usize - 8], dest, src1, src2)
        }

        else {
            format!("{} r{}", ARITHMETIC_IMMEDIATE_OPCODES[opcode as usize - 8], src2)
        }
    }
}

pub fn disassemble_compare (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let condition_code = opcode & 0x7;
    let src1 = (instr >> 20) & 0xF;
    let src2 = (instr >> 16) & 0xF; // For I-type opcodes, this is a shift amount, NOT src2

    match opcode {
        0b0000..=0b0100 => { // Immediate compare instructions
            let mut imm = instr & 0xFFF; // Fetch 12 bit immediate
            imm = ((imm as i32) << 20 >> 20) as u32; // Sign extend it
            imm <<= src2; // Shift the immediate by the shift amount specified in src2

            format!("cmpi/{} r{}, #0x{:08X}", CONDITION_CODES[condition_code as usize], src1, imm)
        } 

        0b1000..=0b1100 => {
            format!("cmp/{} r{}, r{}", CONDITION_CODES[condition_code as usize], src1, src2)
        }

        _ => {
            todo!("Undefined compare opcode: {:01X}", opcode)
        }

        _ => todo!("Unimplemented compare instruction")
    }
}

pub fn disassemble_logical (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let src1 = (instr >> 20) & 0xF;
    let src2 = (instr >> 16) & 0xF; // For I-type opcodes, this is a shift amount, NOT src2
    let dest = (instr >> 12) & 0xF;

    if opcode <= 0b11 {// Opcodes 0 to 3 are I-type and use 12-bit immediates embedded inside the instruction
        let mut imm = instr & 0xFFF; // Fetch 12 bit immediate
        imm = ((imm as i32) << 20 >> 20) as u32; // Sign extend it
        imm <<= src2; // Shift the immediate by the shift amount specified in src2

        format!("{} r{}, r{}, #0x{:08X}", LOGICAL_IMMEDIATE_OPCODES[opcode as usize], dest, src1, imm)
    }

    else {
        match opcode {
            0b100..=0b111 => {
                todo!("Non-existent logical op?")
            }

            0b1000..=0b1011 => {
                format!("{} r{}, r{}, r{}", LOGICAL_REGISTER_OPCODES[opcode as usize - 8], dest, src1, src2)    
            }

            _ => {
                format!("{} r{}, r{}", LOGICAL_REGISTER_OPCODES[opcode as usize - 8], dest, src1)
            }
        }
    }
}