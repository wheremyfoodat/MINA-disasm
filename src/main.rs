#![warn(clippy::all)]
#![allow(clippy::verbose_bit_mask, clippy::new_without_default)]

pub mod cpu;
pub mod helpers;

use crate::cpu::disasm::*;
use crate::helpers::readFileIntoVec;

fn main() {
    let binary = readFileIntoVec(&"ROMs/mov.mina".to_string());
    let mut instruction_index = 0;

    while instruction_index < binary.len() {
        let instruction = u32::from_le_bytes (
            [
                binary[instruction_index], binary[instruction_index+1],
                binary[instruction_index+2], binary[instruction_index+3]
            ]
        );

        println!("{:08X}: {}", instruction_index, disassemble(instruction));
        instruction_index += 4;
    }
}
