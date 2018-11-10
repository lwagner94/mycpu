mod cpu;
mod memory;
mod instruction;

use cpu::write_byte_to_stdout;

fn main() {
    println!("Hello, world!");
    write_byte_to_stdout(67);
}
