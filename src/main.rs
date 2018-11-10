#![allow(dead_code)]

mod cpu;
mod memory;
mod instruction;

use cpu::CPU;
use memory::Memory;

fn main() {
    let memory = Memory::new(1024);
    let mut _cpu = CPU::new(memory);
}
