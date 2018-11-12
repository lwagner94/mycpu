#![allow(dead_code)]

extern crate mycpu;

use std::env;

use mycpu::emulator::cpu::CPU;
use mycpu::emulator::memory::Memory;
use mycpu::common::encoding::DecodedInstruction;
use mycpu::common::generated::instruction::Instruction;
use mycpu::assembler::assembler::assemble_file;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let bytes = assemble_file(&args[1]).unwrap();

    let mut memory = Memory::new(1024);

    memory.write_all(bytes.as_slice(), 0);

    let mut cpu = CPU::new(memory);
    cpu.run();
    cpu.print_state();
}
