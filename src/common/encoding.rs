use byteorder::{BigEndian, WriteBytesExt};

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

    pub fn invalid() -> Self {
        DecodedInstruction::new(Instruction::Invalid, 0, 0, 0, 0)
    }

    pub fn decode(instruction: &[u8; 8]) -> Self {
        let instruction_type = Instruction::from(instruction[0]);
        
        let b0 = instruction[4] as u32;
        let b1 = instruction[5] as u32;
        let b2 = instruction[6] as u32;
        let b3 = instruction[7] as u32;

        let operand = b0 << 24 | b1 << 16 | b2 << 8 | b3;

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

