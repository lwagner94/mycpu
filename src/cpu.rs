use std::num::Wrapping;

use ::instruction::Instruction;
use ::instruction::Instruction::{*};

use ::memory::Memory;

#[derive(Debug)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    PC,
    SP,
    SR
}

pub struct CPU {
    pub regs: [Wrapping<u32>; 19],
    pub memory: Memory
}

impl CPU {
    fn new(mem: Memory) -> Self {
        CPU {
            regs: [Wrapping(0u32); 19],
            memory: mem
        }
    }

    pub fn get_register(self: &Self, reg: Register) -> u32 {
        self.regs[reg as usize].0
    }

    pub fn set_register(self: &mut Self, reg: Register, value: u32) {
        self.regs[reg as usize] = Wrapping(value);
    }

    // This temporary?
    pub fn execute_instruction(self: &mut Self, instruction: Instruction) {
        match instruction {
            LoadImmediate(reg_nr, value) => self.regs[reg_nr] = Wrapping(value),
            Increment(reg_nr) => self.regs[reg_nr] += Wrapping(1),
            Decrement(reg_nr) => self.regs[reg_nr] -= Wrapping(1),
            Add(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] + self.regs[op_2],
            Subtract(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] - self.regs[op_2],
            Multiply(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] * self.regs[op_2],
            Divide(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] / self.regs[op_2],
            And(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] & self.regs[op_2],
            Or(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] | self.regs[op_2],
            XOr(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] ^ self.regs[op_2],
            Negate(reg_nr) => self.regs[reg_nr] = Wrapping((-(self.regs[reg_nr].0 as i64)) as u32),
            Complement(reg_nr) => self.regs[reg_nr] = !self.regs[reg_nr]
        }
    }
}


pub fn write_byte_to_stdout(byte: u8) {
    print!("{}", byte as char);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Register::*;

    fn create_cpu() -> CPU {
        CPU::new(Memory::new())
    }

    fn cpu_arith_prep() -> CPU {
        let mut cpu = create_cpu();
        cpu.set_register(R1, 10);
        cpu.set_register(R2, 5);
        cpu
    }

    fn cpu_binary_prep() -> CPU {
        let mut cpu = create_cpu();
        cpu.set_register(R1, 0b1010);
        cpu.set_register(R2, 0b1101);
        cpu
    }

    #[test]
    fn test_load_immediate() {
        let mut cpu = create_cpu();
        cpu.execute_instruction(LoadImmediate(0, 1337));
        assert_eq!(cpu.get_register(R0), 1337);
    }

    #[test]
    fn test_increment() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Increment(1));
        assert_eq!(cpu.get_register(R1), 11);
    }

    #[test]
    fn test_decrement() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Decrement(1));
        assert_eq!(cpu.get_register(R1), 9);
    }

    #[test]
    fn test_add() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Add(0, 1, 2));
        assert_eq!(cpu.get_register(R0), 15);
    }

    #[test]
    fn test_subtract() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Subtract(0, 1, 2));
        assert_eq!(cpu.get_register(R0), 5);
    }

    #[test]
    fn test_multiply() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Multiply(0, 1, 2));
        assert_eq!(cpu.get_register(R0), 50);
    }

    #[test]
    fn test_divide() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Divide(0, 1, 2));
        assert_eq!(cpu.get_register(R0), 2);
    }

    #[test]
    fn test_or() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(Or(0, 1, 2));
        assert_eq!(cpu.get_register(R0), 0b1111);
    }

    #[test]
    fn test_and() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(And(0, 1, 2));
        assert_eq!(cpu.get_register(R0), 0b1000);
    }

    #[test]
    fn test_xor() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(XOr(0, 1, 2));
        assert_eq!(cpu.get_register(R0), 0b0111);
    }

    #[test]
    fn test_negate() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 0b00000000_00000001);
        cpu.execute_instruction(Negate(0));
        assert_eq!(cpu.get_register(R0), 0b11111111_11111111_11111111_11111111);
    }

    #[test]
    fn test_complement() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 0b00000000_00000001);
        cpu.execute_instruction(Complement(0));
        assert_eq!(cpu.get_register(R0), 0b11111111_11111111_11111111_11111110);
    }



}

