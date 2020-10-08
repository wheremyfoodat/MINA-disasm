const REG_BRANCH_OPCODES: &[&str] = &[
    "rbra", "rcall", "ret", "robra", "rocall"
];

const RELATIVE_BRANCH_OPCODES: &[&str] = &[
    "bra", "bt", "bf", "call", "ct", "cf"
];

pub fn disassemble_reg_branch (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let src1 = (instr >> 20) & 0xF;
    let src2 = (instr >> 16) & 0xF; // For I-type opcodes, this is a shift amount, NOT src2

    match opcode {
        0b10 => "ret".to_string(),
        0 | 1 => {
            let mut imm = instr & 0xFFF; // Fetch 12 bit immediate
            imm = ((imm as i32) << 20 >> 20) as u32; // Sign extend it
            imm <<= (src2 + 2); // Shift the immediate by the shift amount specified in src2 + 2 (to ensure alignment)

            format!("{} r{}, #0x{:08X}", REG_BRANCH_OPCODES[opcode as usize], src1, imm)
        }

        8 | 9 => {
            format!("{}, r{}, r{}", REG_BRANCH_OPCODES[opcode as usize - 6], src1, src2)
        }

        _ => todo!("Undefined register branch opcode!")
    }
}

pub fn disassemble_relative_branch (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let mut imm = instr & 0xFFFFFF;

    imm = ((imm as i32) << 8 >> 8) as u32; // sign extend immediate
    imm <<= 2; // Shift immediate by 2 to ensure alignment

    match opcode {
        0..=2 => {
            format!("{} #0x{:08X}", RELATIVE_BRANCH_OPCODES[opcode as usize], imm)
        }

        8..=10 => {
            format!("{} #0x{:08X}", RELATIVE_BRANCH_OPCODES[opcode as usize - 5], imm)
        }

        _ => todo!("Undefined relative branch opcode!")
    }
}