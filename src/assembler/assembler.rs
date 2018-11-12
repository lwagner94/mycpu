use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use assembler::tokenizer;
use assembler::parser::{parse, ParsedInstruction};
use common::encoding::DecodedInstruction;


pub fn assemble_file(path: &str) -> Option<Vec<u8>> {
    let mut reader = BufReader::new(File::open(path).ok()?);
    let tokens = tokenizer::tokenize(&mut reader);

    let parsed = parse(tokens)?;

    let mut bytes = Vec::new();

    for instr in parsed {
        match instr {
            ParsedInstruction::Instruction(dec) => bytes.write(&dec.encode()).ok()?
        };
    }

    Some(bytes)
}