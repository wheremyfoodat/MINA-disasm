pub mod alu_disasm;
pub mod move_disasm;
pub mod branch_disasm;
pub mod shifts_disasm;
pub mod control_disasm;
pub mod mem_access_disasm;

use crate::cpu::alu_disasm::*;
use crate::cpu::branch_disasm::*;
use crate::cpu::shifts_disasm::*;
use crate::cpu::control_disasm::*;
use crate::cpu::mem_access_disasm::*;
use crate::cpu::move_disasm::*; 


pub fn disassemble(instr: u32) -> String {
    let group = instr >> 28;
    match group {
        0 => disassemble_arithmetic(instr),
        1 => disassemble_logical(instr),
        2 => disassemble_compare(instr),
        3 => disassemble_reg_branch(instr),
        4 => disassemble_mem_access(instr),
        5 => disassemble_mov(instr),
        6 => disassemble_shift (instr),
        7 => disassemble_control(instr),
        8 => disassemble_relative_branch(instr),
        _ => todo!("Unimplemented instruction group! Group code: {}", group)
    }
}