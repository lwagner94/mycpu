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
                if lookup.insert(name.clone(), counter).is_some() {
                    return None;
                }
            }
        }
    }

    lookup.insert(String::from("PROGRAM_START"), MEMORY_START);
    lookup.insert(String::from("PROGRAM_END"), counter);
    lookup.insert(String::from("MEMORY_END"), align_down(MEMORY_END));
    lookup.insert(String::from("CONSOLEIO_START"), CONSOLEIO_START);

    let mut bytes = Vec::new();

    for instr in &parsed {
        match instr {
            ParsedLine::Instruction(dec) => {
                let a: u32 = match &dec.op {
                    Op::Label(name) => *(lookup.get(name)?),
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

#[allow(unused)]
fn align_up(addr: u32) -> u32 {
    let remainder = addr % 4;
    if remainder != 0 {
        addr + (4 - remainder)
    } else {
        addr
    }
}

fn align_down(addr: u32) -> u32 {
    addr - addr % 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_unchanged() {
        let aligned = 16u32;
        assert_eq!(aligned, align_up(aligned));
        assert_eq!(aligned, align_down(aligned));
    }

    #[test]
    fn test_align_up() {
        let n = 17u32;
        assert_eq!(20, align_up(n));
    }

    #[test]
    fn test_align_down() {
        let n = 17u32;
        assert_eq!(16, align_down(n));
    }

}
