#![allow(dead_code)]

extern crate regex;
//#[macro_use] extern crate failure;
extern crate failure;

extern crate mycpu;

use std::fs::File;
use std::io::BufReader;
use std::env;

use mycpu::assembler::tokenizer;
use mycpu::assembler::parser::parse;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1]).unwrap());
    let tokens = tokenizer::tokenize(&mut reader);

    let parsed = parse(tokens);

    println!("{:#?}", parsed);
}

