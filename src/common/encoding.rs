use byteorder::{BigEndian, WriteBytesExt};

use crate::common::generated::instruction::Instruction;
use crate::common::util;

#[derive(Debug)]
pub struct DecodedInstruction {
    pub instruction_type: Instruction,
    pub reg_1: u8,
    pub reg_2: u8,
    pub reg_3: u8,
    pub operand: u32,
}

impl DecodedInstruction {
    pub fn new(
        instruction_type: Instruction,
        reg_1: u8,
        reg_2: u8,
        reg_3: u8,
        operand: u32,
    ) -> Self {
        DecodedInstruction {
            instruction_type,
            reg_1,
            reg_2,
            reg_3,
            operand,
        }
    }

    pub fn invalid() -> Self {
        DecodedInstruction::new(Instruction::Invalid, 0, 0, 0, 0)
    }

    pub fn decode(instruction: [u8; 8]) -> Self {
        let instruction_type = Instruction::from(instruction[0]);

        DecodedInstruction::new(
            instruction_type,
            instruction[1],
            instruction[2],
            instruction[3],
            util::bytes_to_u32(
                instruction[4],
                instruction[5],
                instruction[6],
                instruction[7],
            ),
        )
    }

    pub fn encode(self) -> [u8; 8] {
        let t: u8 = self.instruction_type.into();

        let mut operand_bytes = Vec::new();
        operand_bytes.write_u32::<BigEndian>(self.operand).unwrap();

        [
            t,
            self.reg_1,
            self.reg_2,
            self.reg_3,
            operand_bytes[0],
            operand_bytes[1],
            operand_bytes[2],
            operand_bytes[3],
        ]
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(1, 1);
    }
}
