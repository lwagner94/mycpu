use std::str::FromStr;
use crate::assembler::tokenizer::TokenizedLine;
use crate::assembler::generated::matcher;
use crate::common::generated::instruction::Instruction;

#[derive(Debug)]
pub enum ParsedLine {
    Instruction(MatchedInstruction),
    Label(String)
}

#[derive(Debug)]
pub enum Op {
    Number(u32),
    Label(String)
}


#[derive(Debug)]
pub struct MatchedInstruction {
    pub instruction: Instruction,
    pub reg1: u8,
    pub reg2: u8,
    pub reg3: u8,
    pub op: Op
}


impl MatchedInstruction {
    pub fn new(instruction: Instruction, reg1: u8, reg2: u8, reg3: u8, op: Op) -> Self {
        MatchedInstruction {
            instruction,
            reg1,
            reg2,
            reg3,
            op
        }
    }
}

pub fn parse_register_name(name: &str) -> Option<u8> {
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

pub fn parse_operand(s: &str) -> Option<Op> {
    if let Some(number) = parse_numeric_literal(s) {
        Some(Op::Number(number))
    }
        else {
            Some(Op::Label(s.into()))
        }
}


pub fn parse_label(token: &TokenizedLine) -> Option<ParsedLine> {
    if token.tokens.len() == 1 && token.tokens[0].token.ends_with(':') {
        let label = token.tokens[0].token.replace(':', "");
        Some(ParsedLine::Label(label))
    }
    else {
        None
    }
}


pub fn parse(tokens: Vec<TokenizedLine>) -> Option<Vec<ParsedLine>> {
    let mut parsed_instructions = Vec::new();

    for token in tokens {
        if let Some(instr) = parse_label(&token) {
            parsed_instructions.push(instr);
        }
        else if let Some(instr) = matcher::match_instruction(&token) {
            parsed_instructions.push(instr);
        }
        else {
            return None;
        }
    }

    Some(parsed_instructions)
}

