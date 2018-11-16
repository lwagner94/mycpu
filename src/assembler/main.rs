extern crate mycpu;

use std::env;

use mycpu::assembler::codegen::assemble_file;


fn main() {
    let args: Vec<String> = env::args().collect();
    let bytes = assemble_file(&args[1]).unwrap();

    println!("{:#?}", bytes);
}

