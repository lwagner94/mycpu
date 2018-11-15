use std::num::Wrapping;

use common::encoding::DecodedInstruction;
use common::generated::instruction::Instruction::{*};

use emulator::constants::*;
use emulator::memory::{Memory};
use emulator::memory::AddressSpace;


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
    pub memory: AddressSpace,
    halt: bool,
    instruction: DecodedInstruction
}

impl CPU {
    pub fn new(memory: AddressSpace) -> Self {
        let mut cpu = CPU {
            regs: [Wrapping(0u32); 19],
            memory,
            halt: false,
            instruction: DecodedInstruction::invalid()
        };

        cpu.regs[Register::PC as usize] = Wrapping(MEMORY_START);
        cpu
    }

    fn get_register(self: &Self, reg: Register) -> u32 {
        self.regs[reg as usize].0
    }

    fn set_register(self: &mut Self, reg: Register, value: u32) {
        self.regs[reg as usize] = Wrapping(value);
    }

    pub fn print_state(&self) {
        eprintln!("{:#?}", self.regs);
    }

    pub fn run(self: &mut Self) {
        // Init

        while !self.halt {
            let instruction = self.load_instruction();
            let decoded_instruction = DecodedInstruction::decode(&instruction);
            self.execute_instruction(decoded_instruction);
        }
    }

    fn load_instruction(self: &mut Self) -> [u8; 8] {
        let pc = self.regs[Register::PC as usize].0;
        self.regs[Register::PC as usize] += Wrapping(8);

        let instruction_vec = self.memory.read_all(pc, 8);
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&instruction_vec);

        buffer
    }

    fn execute_instruction(self: &mut Self, d: DecodedInstruction) {

        let reg_1 = d.reg_1 as usize;
        let reg_2 = d.reg_2 as usize;
        let reg_3 = d.reg_3 as usize;

        match d.instruction_type {
            NOp => {},
            Halt => self.halt(),

            Increment => self.regs[reg_1] += Wrapping(1),
            Decrement => self.regs[reg_1] -= Wrapping(1),
            Add => self.regs[reg_1] = self.regs[reg_2] + self.regs[reg_3],
            Subtract => self.regs[reg_1] = self.regs[reg_2] - self.regs[reg_3],
            Multiply => self.regs[reg_1] = self.regs[reg_2] * self.regs[reg_3],
            Divide => self.regs[reg_1] = self.regs[reg_2] / self.regs[reg_3],

            And => self.regs[reg_1] = self.regs[reg_2] & self.regs[reg_3],
            Or => self.regs[reg_1] = self.regs[reg_2] | self.regs[reg_3],
            XOr => self.regs[reg_1] = self.regs[reg_2] ^ self.regs[reg_3],
            Negate => self.regs[reg_1] = Wrapping((-(self.regs[reg_1].0 as i64)) as u32),
            Complement => self.regs[reg_1] = !self.regs[reg_1],

            LoadImmediate => self.regs[reg_1] = Wrapping(d.operand),
            Load => self.regs[reg_1] = Wrapping(self.memory.read_doubleword(d.operand)),
            LoadByte => self.regs[reg_1] = Wrapping(self.memory.read(d.operand) as u32),
            Store => self.memory.write_doubleword(d.operand, self.regs[reg_1].0),
            StoreByte => self.memory.write(d.operand, self.regs[reg_1].0 as u8),
            Push => self.push(reg_1),
            Pop => self.pop(reg_1),

            Jump => self.regs[Register::PC as usize] = Wrapping(d.operand),
            Call => self.call(d.operand),
            Return => self.return_from_call(),

            Invalid => panic!("Invalid instruction {:?}", d.instruction_type)
        }
    }

    fn push(&mut self, register: usize) {
        self.regs[Register::SP as usize] -= Wrapping(4);
        self.memory.write_doubleword(self.regs[Register::SP as usize].0, self.regs[register].0);
    }

    fn pop(&mut self, register: usize) {
        self.regs[register] = Wrapping(self.memory.read_doubleword(self.regs[Register::SP as usize].0));
        self.regs[Register::SP as usize] += Wrapping(4);
    }

    fn call(&mut self, address: u32) {
        self.push(Register::PC as usize);
        self.regs[Register::PC as usize] = Wrapping(address);
    }

    fn return_from_call(&mut self) {
        let pc = self.pop(Register::PC as usize);
    }

    fn halt(self: &mut Self) {
        eprintln!("Halting CPU at PC=0x{:X}", self.get_register(Register::PC));
        self.halt = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Register::*;

    fn create_cpu() -> CPU {
        let addr_space = AddressSpace::default();

        CPU::new(addr_space)
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
        cpu.execute_instruction(DecodedInstruction::new(
            LoadImmediate, 0, 0, 0, 1337));
        assert_eq!(cpu.get_register(R0), 1337);
    }

    #[test]
    fn test_increment() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            Increment, 1, 0,0,0));
        assert_eq!(cpu.get_register(R1), 11);
    }

    #[test]
    fn test_decrement() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            Decrement, 1, 0,0,0));
        assert_eq!(cpu.get_register(R1), 9);
    }

    #[test]
    fn test_add() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            Add, 0, 1,2,0));
        assert_eq!(cpu.get_register(R0), 15);
    }

    #[test]
    fn test_subtract() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            Subtract, 0, 1,2,0));
        assert_eq!(cpu.get_register(R0), 5);
    }

    #[test]
    fn test_multiply() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            Multiply, 0, 1,2,0));
        assert_eq!(cpu.get_register(R0), 50);
    }

    #[test]
    fn test_divide() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            Divide, 0, 1,2,0));
        assert_eq!(cpu.get_register(R0), 2);
    }

    #[test]
    fn test_or() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            Or, 0, 1,2,0));
        assert_eq!(cpu.get_register(R0), 0b1111);
    }

    #[test]
    fn test_and() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            And, 0, 1,2,0));
        assert_eq!(cpu.get_register(R0), 0b1000);
    }

    #[test]
    fn test_xor() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(DecodedInstruction::new(
            XOr, 0, 1,2,0));
        assert_eq!(cpu.get_register(R0), 0b0111);
    }

    #[test]
    fn test_negate() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 0b00000000_00000001);
        cpu.execute_instruction(DecodedInstruction::new(
            Negate, 0, 0,0,0));
        assert_eq!(cpu.get_register(R0), 0b11111111_11111111_11111111_11111111);
    }

    #[test]
    fn test_complement() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 0b00000000_00000001);
        cpu.execute_instruction(DecodedInstruction::new(
            Complement, 0, 0,0,0));
        assert_eq!(cpu.get_register(R0), 0b11111111_11111111_11111111_11111110);
    }
}