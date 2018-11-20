
// AUTOMATICALLY GENERATED, DO NOT EDIT!
use std::mem::transmute;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Instruction {
    NOp,
    Halt,
    Increment,
    Decrement,
    Add,
    Subtract,
    Multiply,
    Divide,
    Compare,
    CompareImmediate,
    Or,
    And,
    XOr,
    Negate,
    Complement,
    LoadImmediate,
    Load,
    LoadByte,
    LoadDirect,
    LoadDirectByte,
    Store,
    StoreByte,
    StoreDirect,
    StoreDirectByte,
    Push,
    Pop,
    Jump,
    Call,
    Return,
    BranchEqual,
    BranchNotEqual,
    Move,
    Invalid,

}

impl Into<u8> for Instruction {
    fn into(self: Self) -> u8 {
        self as u8
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Instruction {
        unsafe {transmute(value)}
    }
}
