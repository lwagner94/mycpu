extern crate mycpu;

use std::env;
use std::time::SystemTime;

use mycpu::emulator::cpu::CPU;
use mycpu::emulator::memory::{AddressSpace, Memory};
use mycpu::assembler::codegen::assemble_file;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let bytes = assemble_file(&args[1]).unwrap();

    let mut memory = AddressSpace::default();

    memory.write_all(bytes.as_slice(), 0x10_0000);

    let mut cpu = CPU::new(memory);
    let before = SystemTime::now();
    cpu.run();
    let after = SystemTime::now();
    cpu.print_state();

    let elapsed = after.duration_since(before).unwrap().as_secs();
    eprintln!("Executed {} instructions in {:?} seconds", cpu.cycle_counter, elapsed);
    eprintln!("Frequency: {} MHz", cpu.cycle_counter as f64 / elapsed as f64/ 1000.0 / 1000.0);
}
