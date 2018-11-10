
#[derive(Debug)]
pub enum Instruction {
    NOp,
    Invalid,

    /* Arithmetic */
    Increment,
    Decrement,
    Add,
    Subtract,
    Multiply,
    Divide,

    /* Logic */
    Or,
    And,
    XOr,
    Negate,
    Complement,

    /* Load */
    LoadImmediate,
}

impl Into<u8> for Instruction {
    fn into(self: Self) -> u8 {
        match self {
            NOp => 0x00,

            /* Arithmetic */
            Increment => 0x10,
            Decrement => 0x11,
            Add => 0x12,
            Subtract => 0x13,
            Multiply => 0x14,
            Divide => 0x15,

            /* Logic */
            Or => 0x20,
            And => 0x21,
            XOr => 0x22,
            Negate => 0x23,
            Complement => 0x24,

            /* Load */
            LoadImmediate => 0x30,

            Invalid => 0xFF,
        }
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Instruction {
        match value {
            0x00 => NOp,

            /* Arithmetic */
            0x10 => Increment,
            0x11 => Decrement,
            0x12 => Add,
            0x13 => Subtract,
            0x14 => Multiply,
            0x15 => Divide,

            /* Logic */
            0x20 => Or,
            0x21 => And,
            0x22 => XOr,
            0x23 => Negate,
            0x24 => Complement,

            /* Load */
            0x30 => LoadImmediate,

            _ => Invalid
        }
    }
}

use self::Instruction::*;

pub struct DecodedInstruction {
    pub instruction_type: Instruction,
    pub reg_1: u8,
    pub reg_2: u8,
    pub reg_3: u8,
    pub operand: u32
}

impl DecodedInstruction {
    pub fn new(instruction_type: Instruction, reg_1: u8, reg_2: u8, reg_3: u8, operand: u32) -> Self {
        DecodedInstruction {
            instruction_type,
            reg_1,
            reg_2,
            reg_3,
            operand
        }
    }

    /*
    fn no_reg(instruction_type: Instruction) -> Self {
        Self::new(instruction_type, 0,0, 0, 0)
    }

    fn one_reg(instruction_type: Instruction, reg_1: u8) -> Self {
        Self::new(instruction_type, reg_1,0, 0, 0)
    }

    fn three_reg(instruction_type: Instruction, reg_1: u8, reg_2: u8, reg_3: u8) -> Self {
        Self::new(instruction_type, reg_1, reg_2, reg_3, 0)
    }

    fn one_reg_operand(instruction_type: Instruction, reg_1: u8, operand: u32) -> Self {
        Self::new(instruction_type, reg_1, 0, 0, operand)
    }
    */

    pub fn decode(instruction: &[u32; 2]) -> DecodedInstruction {
        let mut mask = 0xFF_00_00_00u32;
        let instruction_type = Instruction::from(((instruction[0] & mask) >> 24) as u8);

        mask = 0x00_FF_00_00u32;
        let reg_1 = ((instruction[0] & mask) >> 16) as u8;

        mask = 0x00_00_FF_00u32;
        let reg_2 = ((instruction[0] & mask) >> 8) as u8;

        mask = 0x00_00_00_FFu32;
        let reg_3 = ((instruction[0] & mask) >> 0) as u8;

        let operand = instruction[1];

        DecodedInstruction::new(instruction_type, reg_1, reg_2, reg_3, operand)
    }

    pub fn encode(instruction_type: Instruction,
                  reg_1: u8, reg_2: u8, reg_3: u8, operand: u32) -> [u32; 2] {

        let t: u8 = instruction_type.into();
        [(t as u32) << 24 | (reg_1 as u32) << 16 | (reg_2 as u32) << 8 | reg_3 as u32 , operand]
    }
}






#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(1,1);
    }
}

