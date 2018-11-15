#![allow(dead_code)]

extern crate mycpu;

use std::env;

use mycpu::emulator::cpu::CPU;
use mycpu::emulator::memory::{AddressSpace, Memory};
use mycpu::assembler::assembler::assemble_file;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let bytes = assemble_file(&args[1]).unwrap();

    let mut memory = AddressSpace::default();

    memory.write_all(bytes.as_slice(), 0x100000);

    let mut cpu = CPU::new(memory);
    cpu.run();
    cpu.print_state();
}
