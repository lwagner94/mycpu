#![allow(dead_code)]

mod cpu;
mod memory;
mod instruction;

use cpu::CPU;
use memory::Memory;
use instruction::DecodedInstruction;
use instruction::Instruction;

fn main() {
    let mut memory = Memory::new(1024);

    let d = DecodedInstruction::new(Instruction::Halt, 0, 0, 0, 0);
    memory.write_instruction(32, d.encode());

    let mut cpu = CPU::new(memory);
    cpu.run();
}
