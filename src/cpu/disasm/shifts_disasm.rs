const SHIFT_OPCODES: &[&str] = &[
    "lsl", "lsr", "asr", "ror", "rlsl", "rlsr",
    "rasr", "rror", "flsl", "flsr"
];

pub fn disassemble_shift (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let src1 = (instr >> 20) & 0xF;
    let src2 = (instr >> 16) & 0xF; // For I-type opcodes, this is a shift amount, NOT src2
    let dest = (instr >> 12) & 0xF;
    let rshift = (instr >> 8) & 0xF;

    match opcode {
        0..=3 => {
            format!("{} r{}, r{}, #{}", SHIFT_OPCODES[opcode as usize], dest, src1, src2)
        }

        8..=11 => {
            format!("{} r{}, r{}, r{}", SHIFT_OPCODES[opcode as usize - 4], dest, src1, src2)
        }

        12 | 13 => {
            format!("{} r{}, r{}, r{}, r{}", SHIFT_OPCODES[opcode as usize - 4], dest, src1, src2, rshift)
        }

        _ => todo!("Undefined shift instruction!")
    }
}