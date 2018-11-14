use common::generated::instruction::Instruction;
use assembler::tokenizer::TokenizedLine;
use assembler::parser::{*};

pub fn match_instruction(line: &TokenizedLine) -> Option<ParsedLine> {
    if line.tokens.len() == 0 {
        return None;
    }
    let instruction_identifier = line.tokens[0].token.as_str();

    let dec = match instruction_identifier {
 
        "nop" if line.tokens.len() == 1 => MatchedInstruction::new(
            Instruction::NOp,
            0,
            0,
            0,
            Op::Number(0)), 
        "halt" if line.tokens.len() == 1 => MatchedInstruction::new(
            Instruction::Halt,
            0,
            0,
            0,
            Op::Number(0)), 
        "inc" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Increment,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            Op::Number(0)), 
        "dec" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Decrement,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            Op::Number(0)), 
        "add" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::Add,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            parse_register_name(line.tokens[3].token.as_str())?,
            Op::Number(0)), 
        "sub" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::Subtract,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            parse_register_name(line.tokens[3].token.as_str())?,
            Op::Number(0)), 
        "mul" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::Multiply,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            parse_register_name(line.tokens[3].token.as_str())?,
            Op::Number(0)), 
        "div" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::Divide,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            parse_register_name(line.tokens[3].token.as_str())?,
            Op::Number(0)), 
        "or" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::Or,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            parse_register_name(line.tokens[3].token.as_str())?,
            Op::Number(0)), 
        "and" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::And,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            parse_register_name(line.tokens[3].token.as_str())?,
            Op::Number(0)), 
        "xor" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::XOr,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            parse_register_name(line.tokens[3].token.as_str())?,
            Op::Number(0)), 
        "neg" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Negate,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            Op::Number(0)), 
        "com" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Complement,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            Op::Number(0)), 
        "ldi" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::LoadImmediate,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            parse_operand(line.tokens[2].token.as_str())?), 
        "ld" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::Load,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            parse_operand(line.tokens[2].token.as_str())?), 
        "st" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::Store,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            parse_operand(line.tokens[2].token.as_str())?), 
        "push" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Push,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            Op::Number(0)), 
        "pop" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Pop,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            Op::Number(0)), 
        "jmp" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Jump,
            0,
            0,
            0,
            parse_operand(line.tokens[1].token.as_str())?), 
        "invalid" if line.tokens.len() == 1 => MatchedInstruction::new(
            Instruction::Invalid,
            0,
            0,
            0,
            Op::Number(0)),
        _ => return None
    };


    Some(ParsedLine::Instruction(dec))
}