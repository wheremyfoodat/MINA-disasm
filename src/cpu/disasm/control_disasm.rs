const CONTROL_OPCODES: &[&str] = &[
    "stop", "wfi", "sett", "clrt", "switch",
    "svcall", "fault", "mtof", "mfrf"
];

pub fn disassemble_control (instr: u32) -> String {
    let opcode = (instr >> 24) & 0xF;
    let src1 = (instr >> 20) & 0xF;
    let dest = (instr >> 12) & 0xF;

    match opcode {
        0..=0b100 => {
            CONTROL_OPCODES[opcode as usize].to_string()
        }

        0b1001 => {
            format!("fault r{}, r{}", dest, src1)
        }

        0b1000 | 0b1010 | 0b1011 => {
            format!("{} r{}", CONTROL_OPCODES[opcode as usize - 3], dest) 
        }

        _ => todo!("Undefined control instruction!")
    }
}