const MEM_OPCODES: &[&str] = &[
    "ld", "ldh", "ldb", "st", "sth",
    "stb", "ldc", "stc", "rld", "rldh",
    "rldb", "rst", "rsth", "rstb", "pop", "push"
];

pub fn disassemble_mem_access (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let src1 = (instr >> 20) & 0xF;
    let src2 = (instr >> 16) & 0xF; // For I-type opcodes, this is a shift amount, NOT src2
    let dest = (instr >> 12) & 0xF;

    match opcode {
        0000..=0b0111 => {
            let mut imm = instr & 0xFFF; // Fetch 12 bit immediate
            imm = ((imm as i32) << 20 >> 20) as u32; // Sign extend it
            imm <<= src2; // Shift the immediate by the shift amount specified in src2

            if opcode == 0 || opcode == 3 || opcode == 6 || opcode == 7 { // add 2 to the shift amount for LD, ST, LDC, STC
                imm <<= 2;
            }

            else if opcode == 1 || opcode == 4 { // add 1 to the shift amount for LDH/STH
                imm <<= 1;
            }

            if opcode == 6 || opcode == 7 { // LDC/STC
                return format!("{} [r{}, #0x{:08X}]", MEM_OPCODES[opcode as usize], src1, imm)
            }

            format!("{} r{}, [r{}, #0x{:08X}]", MEM_OPCODES[opcode as usize], dest, src1, imm)
        }

        0b1000..=0b1101 => {
            format!("{} r{}, [r{}, r{}]", MEM_OPCODES[opcode as usize], dest, src1, src2)
        }

        _ => {
            format!("{} r{}", MEM_OPCODES[opcode as usize], dest)
        }
    }
}