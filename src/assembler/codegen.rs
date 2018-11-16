use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::u32;

use crate::assembler::parser::{parse, Op, ParsedLine};
use crate::assembler::tokenizer;
use crate::common::encoding::DecodedInstruction;
use crate::emulator::constants::*;

pub fn assemble_file(path: &str) -> Option<Vec<u8>> {
    let mut reader = BufReader::new(File::open(path).ok()?);
    let tokens = tokenizer::tokenize(&mut reader);

    let parsed = parse(tokens)?;

    let mut lookup = HashMap::new();
    let mut counter = MEMORY_START;

    // Build lookup table
    for instr in &parsed {
        match instr {
            ParsedLine::Instruction(_dec) => counter += 8,
            ParsedLine::Label(name) => {
                if lookup.insert(name, counter).is_some() {
                    return None;
                }
            }
        }
    }

    let mut bytes = Vec::new();

    for instr in &parsed {
        match instr {
            ParsedLine::Instruction(dec) => {
                let a: u32 = match &dec.op {
                    Op::Label(name) => *(lookup.get(&name)?),
                    Op::Number(number) => *number,
                };

                let instr = DecodedInstruction::new(
                    dec.instruction.clone(),
                    dec.reg1,
                    dec.reg2,
                    dec.reg3,
                    a,
                );

                bytes.write(&instr.encode()).ok()?;
            }
            ParsedLine::Label(_name) => {}
        };
    }

    Some(bytes)
}
