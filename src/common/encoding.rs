

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

    pub fn encode(self: Self) -> [u32; 2] {

        let t: u8 = self.instruction_type.into();
        [(t as u32) << 24 | (self.reg_1 as u32) << 16 | (self.reg_2 as u32) << 8 | self.reg_3 as u32 , self.operand]
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

