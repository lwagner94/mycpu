#![allow(dead_code)]

extern crate mycpu;

use mycpu::emulator::cpu::CPU;
use mycpu::emulator::memory::Memory;
use mycpu::emulator::encoding::DecodedInstruction;
use mycpu::common::generated::instruction::Instruction;

fn main() {
    let mut memory = Memory::new(1024);

    let d = DecodedInstruction::new(Instruction::Halt, 0, 0, 0, 0);
    memory.write_instruction(32, d.encode());

    let mut cpu = CPU::new(memory);
    cpu.run();
}
