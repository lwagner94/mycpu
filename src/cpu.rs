use ::instruction::Instruction;
use ::instruction::Instruction::{*};


pub struct CPU {
    pub pc: u16,
    pub regs: [u16; 16]
}

impl CPU {
    fn new() -> Self {
        CPU {
            pc: 0,
            regs: [0; 16]
        }
    }

    // This temporary?
    pub fn execute_instruction(self: &mut Self, instruction: Instruction) {
        match instruction {
            LoadImmediate(reg_nr, value) => self.regs[reg_nr] = value,
            Increment(reg_nr) => self.regs[reg_nr] += 1,
            Decrement(reg_nr) => self.regs[reg_nr] -= 1,
            Add(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] + self.regs[op_2],
            Subtract(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] - self.regs[op_2],
            Multiply(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] * self.regs[op_2],
            Divide(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] / self.regs[op_2],
            And(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] & self.regs[op_2],
            Or(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] | self.regs[op_2],
            XOr(dest_reg, op_1, op_2) => self.regs[dest_reg] = self.regs[op_1] ^ self.regs[op_2],
            Negate(reg_nr) => self.regs[reg_nr] = (-(self.regs[reg_nr] as i32)) as u16,
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

    fn create_cpu() -> CPU {
        CPU::new()
    }

    fn cpu_arith_prep() -> CPU {
        let mut cpu = create_cpu();
        cpu.regs[1] = 10;
        cpu.regs[2] = 5;
        cpu
    }

    fn cpu_binary_prep() -> CPU {
        let mut cpu = create_cpu();
        cpu.regs[1] = 0b1010;
        cpu.regs[2] = 0b1101;
        cpu
    }

    #[test]
    fn test_load_immediate() {
        let mut cpu = create_cpu();
        cpu.execute_instruction(LoadImmediate(0, 1337));
        assert_eq!(cpu.regs[0], 1337);
    }

    #[test]
    fn test_increment() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Increment(1));
        assert_eq!(cpu.regs[1], 11);
    }

    #[test]
    fn test_decrement() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Decrement(1));
        assert_eq!(cpu.regs[1], 9);
    }

    #[test]
    fn test_add() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Add(0, 1, 2));
        assert_eq!(cpu.regs[0], 15);
    }

    #[test]
    fn test_subtract() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Subtract(0, 1, 2));
        assert_eq!(cpu.regs[0], 5);
    }

    #[test]
    fn test_multiply() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Multiply(0, 1, 2));
        assert_eq!(cpu.regs[0], 50);
    }

    #[test]
    fn test_divide() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(Divide(0, 1, 2));
        assert_eq!(cpu.regs[0], 2);
    }

    #[test]
    fn test_or() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(Or(0, 1, 2));
        assert_eq!(cpu.regs[0], 0b1111);
    }

    #[test]
    fn test_and() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(And(0, 1, 2));
        assert_eq!(cpu.regs[0], 0b1000);
    }

    #[test]
    fn test_xor() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(XOr(0, 1, 2));
        assert_eq!(cpu.regs[0], 0b0111);
    }

    #[test]
    fn test_negate() {
        let mut cpu = create_cpu();
        cpu.regs[0] = 0b00000000_00000001;
        cpu.execute_instruction(Negate(0));
        assert_eq!(cpu.regs[0], 0b11111111_11111111);
    }

    #[test]
    fn test_complement() {
        let mut cpu = create_cpu();
        cpu.regs[0] = 0b00000000_00000001;
        cpu.execute_instruction(Complement(0));
        assert_eq!(cpu.regs[0], 0b11111111_11111110);
    }



}

