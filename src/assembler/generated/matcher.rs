use crate::common::generated::instruction::Instruction;
use crate::assembler::tokenizer::TokenizedLine;
use crate::assembler::parser::{*};

#[allow(clippy::cyclomatic_complexity)]
pub fn match_instruction(line: &TokenizedLine) -> Option<ParsedLine> {
    if line.tokens.is_empty() {
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
        "cmp" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::Compare,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            Op::Number(0)), 
        "cmpi" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::CompareImmediate,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            parse_operand(line.tokens[2].token.as_str())?), 
        "addi" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::AddImmediate,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            parse_operand(line.tokens[3].token.as_str())?), 
        "subi" if line.tokens.len() == 4 => MatchedInstruction::new(
            Instruction::SubtractImmediate,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            parse_operand(line.tokens[3].token.as_str())?), 
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
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            Op::Number(0)), 
        "ldb" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::LoadByte,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            Op::Number(0)), 
        "ldd" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::LoadDirect,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            parse_operand(line.tokens[2].token.as_str())?), 
        "lddb" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::LoadDirectByte,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            parse_operand(line.tokens[2].token.as_str())?), 
        "st" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::Store,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            Op::Number(0)), 
        "stb" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::StoreByte,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            Op::Number(0)), 
        "std" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::StoreDirect,
            parse_register_name(line.tokens[1].token.as_str())?,
            0,
            0,
            parse_operand(line.tokens[2].token.as_str())?), 
        "stdb" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::StoreDirectByte,
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
        "call" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::Call,
            0,
            0,
            0,
            parse_operand(line.tokens[1].token.as_str())?), 
        "ret" if line.tokens.len() == 1 => MatchedInstruction::new(
            Instruction::Return,
            0,
            0,
            0,
            Op::Number(0)), 
        "breq" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::BranchEqual,
            0,
            0,
            0,
            parse_operand(line.tokens[1].token.as_str())?), 
        "brne" if line.tokens.len() == 2 => MatchedInstruction::new(
            Instruction::BranchNotEqual,
            0,
            0,
            0,
            parse_operand(line.tokens[1].token.as_str())?), 
        "mov" if line.tokens.len() == 3 => MatchedInstruction::new(
            Instruction::Move,
            parse_register_name(line.tokens[1].token.as_str())?,
            parse_register_name(line.tokens[2].token.as_str())?,
            0,
            Op::Number(0)), 
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