const MOVE_OPCODES: &[&str] = &[
    "movi", "mti", "mfi", "movl", "movu",
    "mov", "mt", "mf", "mtoc", "mfrc", 
    "mtou", "mfru"
];

pub fn disassemble_mov (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let src1 = (instr >> 20) & 0xF;
    let src2 = (instr >> 16) & 0xF; // For I-type opcodes, this is a shift amount, NOT src2
    let dest = (instr >> 12) & 0xF;

    match opcode {
        0..=2 => {
            let mut imm = instr & 0xFFF; // Fetch 12 bit immediate
            imm = ((imm as i32) << 20 >> 20) as u32; // Sign extend it
            imm <<= src2; // Shift the immediate by the shift amount specified in src2

            format!("{} r{}, {:08X}", MOVE_OPCODES[opcode as usize], dest, imm)
        }

        3 | 4 => {
            let imm = instr & 0xFFFF;

            format!("{} r{}, {:08X}", MOVE_OPCODES[opcode as usize], dest, imm)
        }

        8..=10 | 13 | 14 => {
            format!("{} r{}, r{}", MOVE_OPCODES[opcode as usize - 3], dest, src1)
        }

        11 | 12 => {
            format!("{} r{}", MOVE_OPCODES[opcode as usize - 3], dest)
        }

        _ => todo!("Undefined move instruction!")
    }
}