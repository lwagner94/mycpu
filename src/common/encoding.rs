use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;

use super::super::common::generated::instruction::Instruction;

#[derive(Debug)]
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

    pub fn decode(instruction: &[u8; 8]) -> DecodedInstruction {
        let instruction_type = Instruction::from(instruction[0]);

        let mut operand_reader = Cursor::new(&instruction[4..=7]);
        let operand = operand_reader.read_u32::<BigEndian>().unwrap();

        DecodedInstruction::new(instruction_type, instruction[1], instruction[2], instruction[3], operand)
    }

    pub fn encode(self) -> [u8; 8] {
        let t: u8 = self.instruction_type.into();

        let mut operand_bytes = Vec::new();
        operand_bytes.write_u32::<BigEndian>(self.operand).unwrap();

        [t, self.reg_1, self.reg_2, self.reg_3,
            operand_bytes[0], operand_bytes[1],
            operand_bytes[2], operand_bytes[3]]
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

