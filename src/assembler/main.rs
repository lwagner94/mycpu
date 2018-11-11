#![allow(dead_code)]

extern crate regex;
#[macro_use] extern crate failure;

extern crate mycpu;

use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;
use std::env;
use failure::Error;

use mycpu::assembler::tokenizer;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1]).unwrap());
    let tokens = tokenizer::tokenize(&mut reader);

    println!("{:#?}", tokens);
}

