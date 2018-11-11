use common::encoding::DecodedInstruction;
use assembler::tokenizer::TokenizedLine;
use assembler::generated::matcher;

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
        assert_eq!(match_register_name("r0").unwrap(), 0);
    }

    #[test]
    fn test_match_register_name_invalid() {
        assert_eq!(match_register_name("rf"), None);
    }
}