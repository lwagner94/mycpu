
// AUTOMATICALLY GENERATED, DO NOT EDIT!

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
    Or,
    And,
    XOr,
    Negate,
    Complement,
    LoadImmediate,
    Load,
    LoadByte,
    Store,
    StoreByte,
    Push,
    Pop,
    Jump,
    Call,
    Return,
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
            Instruction::Compare => 0x16,
            Instruction::Or => 0x20,
            Instruction::And => 0x21,
            Instruction::XOr => 0x22,
            Instruction::Negate => 0x23,
            Instruction::Complement => 0x24,
            Instruction::LoadImmediate => 0x30,
            Instruction::Load => 0x31,
            Instruction::LoadByte => 0x32,
            Instruction::Store => 0x33,
            Instruction::StoreByte => 0x34,
            Instruction::Push => 0x35,
            Instruction::Pop => 0x36,
            Instruction::Jump => 0x40,
            Instruction::Call => 0x41,
            Instruction::Return => 0x42,
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
            0x16 => Instruction::Compare,
            0x20 => Instruction::Or,
            0x21 => Instruction::And,
            0x22 => Instruction::XOr,
            0x23 => Instruction::Negate,
            0x24 => Instruction::Complement,
            0x30 => Instruction::LoadImmediate,
            0x31 => Instruction::Load,
            0x32 => Instruction::LoadByte,
            0x33 => Instruction::Store,
            0x34 => Instruction::StoreByte,
            0x35 => Instruction::Push,
            0x36 => Instruction::Pop,
            0x40 => Instruction::Jump,
            0x41 => Instruction::Call,
            0x42 => Instruction::Return,
            0xff => Instruction::Invalid,

            _ => Instruction::Invalid
        }
    }
}
