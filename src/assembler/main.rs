#![allow(dead_code)]

extern crate mycpu;

use std::env;

use mycpu::assembler::assembler::assemble_file;


fn main() {
    let args: Vec<String> = env::args().collect();
    let bytes = assemble_file(&args[1]).unwrap();

    println!("{:#?}", bytes);
}

