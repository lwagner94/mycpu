
// AUTOMATICALLY GENERATED, DO NOT EDIT!

#[derive(Debug)]
pub enum Instruction {
    NOp,
    Halt,
    Increment,
    Decrement,
    Add,
    Subtract,
    Multiply,
    Divide,
    Or,
    And,
    XOr,
    Negate,
    Complement,
    LoadImmediate,
    Load,
    Store,
    Push,
    Pop,
    Jump,
    Invalid,

}

impl Into<u8> for Instruction {
    fn into(self: Self) -> u8 {
        match self {
            Instruction::NOp => 0x0,
            Instruction::Halt => 0x1,
            Instruction::Increment => 0x10,
            Instruction::Decrement => 0x11,
            Instruction::Add => 0x12,
            Instruction::Subtract => 0x13,
            Instruction::Multiply => 0x14,
            Instruction::Divide => 0x15,
            Instruction::Or => 0x20,
            Instruction::And => 0x21,
            Instruction::XOr => 0x22,
            Instruction::Negate => 0x23,
            Instruction::Complement => 0x24,
            Instruction::LoadImmediate => 0x30,
            Instruction::Load => 0x31,
            Instruction::Store => 0x32,
            Instruction::Push => 0x33,
            Instruction::Pop => 0x34,
            Instruction::Jump => 0x40,
            Instruction::Invalid => 0xff,

        }
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Instruction {
        match value {
            0x0 => Instruction::NOp,
            0x1 => Instruction::Halt,
            0x10 => Instruction::Increment,
            0x11 => Instruction::Decrement,
            0x12 => Instruction::Add,
            0x13 => Instruction::Subtract,
            0x14 => Instruction::Multiply,
            0x15 => Instruction::Divide,
            0x20 => Instruction::Or,
            0x21 => Instruction::And,
            0x22 => Instruction::XOr,
            0x23 => Instruction::Negate,
            0x24 => Instruction::Complement,
            0x30 => Instruction::LoadImmediate,
            0x31 => Instruction::Load,
            0x32 => Instruction::Store,
            0x33 => Instruction::Push,
            0x34 => Instruction::Pop,
            0x40 => Instruction::Jump,
            0xff => Instruction::Invalid,

            _ => Instruction::Invalid
        }
    }
}
