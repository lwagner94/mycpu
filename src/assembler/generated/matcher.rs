use common::generated::instruction::Instruction;
use assembler::tokenizer::TokenizedLine;
use assembler::parser::ParsedInstruction;
use common::encoding::DecodedInstruction;
use assembler::parser::match_register_name;


pub fn match_instruction(line: &TokenizedLine) -> Option<ParsedInstruction> {
    if line.tokens.len() == 0 {
        return None;
    }
    let instruction_identifier = line.tokens[0].token.as_str();

    let dec = match instruction_identifier {
        "add" if line.tokens.len() == 4 => DecodedInstruction::new(
            Instruction::Add,
            match_register_name(line.tokens[1].token.as_str())?,
            match_register_name(line.tokens[2].token.as_str())?,
            match_register_name(line.tokens[3].token.as_str())?,
            0),
        "halt" if line.tokens.len() == 1 => DecodedInstruction::new(
            Instruction::Halt,
            0,
            0,
            0,
            0),
        _ => return None
    };


    Some(ParsedInstruction::Instruction(dec))
}