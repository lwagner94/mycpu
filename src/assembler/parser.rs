use common::encoding::DecodedInstruction;
use assembler::tokenizer::TokenizedLine;
use assembler::generated::matcher;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParsedInstruction {
    Instruction(DecodedInstruction),

}


pub fn match_register_name(name: &str) -> Option<u8> {
    match name {
        "r0" => Some(0),
        "r1" => Some(1),
        "r2" => Some(2),
        "r3" => Some(3),
        "r4" => Some(4),
        "r5" => Some(5),
        "r6" => Some(6),
        "r7" => Some(7),
        "r8" => Some(8),
        "r9" => Some(9),
        "r10" => Some(10),
        "r11" => Some(11),
        "r12" => Some(12),
        "r13" => Some(13),
        "r14" => Some(14),
        "r15" => Some(15),
        "pc" => Some(16),
        "sp" => Some(17),
        "sr" => Some(18),
        _ => None
    }
}

pub fn parse_numeric_literal(literal: &str) -> Option<u32> {
    if literal.starts_with("0x") {
        let without_prefix = literal.trim_left_matches("0x");
        u32::from_str_radix(without_prefix, 16).ok()
    }
    else {
        u32::from_str(literal).ok()
    }
}


pub fn parse(tokens: Vec<TokenizedLine>) -> Option<Vec<ParsedInstruction>> {
//    matcher::
    let mut parsed_instructions = Vec::new();

    for token in tokens {
        let i = matcher::match_instruction(&token)?;
        parsed_instructions.push(i);
    }

    Some(parsed_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_register_name() {
        assert_eq!(match_register_name("r0"), Some(0));
    }

    #[test]
    fn test_match_register_name_invalid() {
        assert_eq!(match_register_name("rf"), None);
    }

    #[test]
    fn test_parse_numeric_literal_decimal() {
        assert_eq!(parse_numeric_literal("100"), Some(100));
    }

    #[test]
    fn test_parse_numeric_literal_hex() {
        assert_eq!(parse_numeric_literal("0xFF"), Some(255));
    }

    #[test]
    fn test_parse_numeric_literal_hex_fail() {
        assert_eq!(parse_numeric_literal("0xFFx0"), None);
    }

    #[test]
    fn test_parse_numeric_literal_fail() {
        assert_eq!(parse_numeric_literal("abc"), None);
    }
}