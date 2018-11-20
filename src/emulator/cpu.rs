use std::num::Wrapping;

use crate::common::encoding::DecodedInstruction;
use crate::common::generated::instruction::Instruction::*;
use crate::emulator::constants::*;
use crate::emulator::memory::AddressSpace;
use crate::emulator::memory::Memory;

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
    SR,
}

#[derive(Debug)]
pub enum StatusBit {
    Zero = 0,
    Negative = 1,
    Carry = 2,
}

pub struct CPU {
    pub regs: [Wrapping<u32>; 19],
    pub memory: AddressSpace,
    halt: bool,
    pub cycle_counter: u64,
}

impl CPU {
    // Status register:
    // Carry, Zero, Negative,

    pub fn new(memory: AddressSpace) -> Self {
        let mut cpu = CPU {
            regs: [Wrapping(0u32); 19],
            memory,
            halt: false,
            cycle_counter: 0,
        };

        cpu.regs[Register::PC as usize] = Wrapping(MEMORY_START);
        cpu
    }

    fn set_status_bit(&mut self, bit: StatusBit, set: bool) {
        let mut value = self.regs[Register::SR as usize].0;

        let mask = 0x1u32 << bit as u32;

        if set {
            value |= mask;
        } else {
            value &= !mask;
        }

        self.regs[Register::SR as usize] = Wrapping(value);
    }

    fn get_status_bit(&self, bit: StatusBit) -> bool {
        let value = self.regs[Register::SR as usize].0;
        let mask = 0x1u32 << bit as u32;

        (value & mask) != 0
    }

    fn get_register(self: &Self, reg: Register) -> u32 {
        self.regs[reg as usize].0
    }

    #[allow(unused)]
    fn set_register(self: &mut Self, reg: Register, value: u32) {
        self.regs[reg as usize] = Wrapping(value);
    }

    pub fn print_state(&self) {
        eprintln!("Registers:");
        eprintln!("{:#?}", self.regs);
    }

    pub fn run(self: &mut Self) {
        // Init

        while !self.halt {
            let instruction = self.load_instruction();
            let decoded_instruction = DecodedInstruction::decode(instruction);
            self.cycle_counter += 1;
            self.execute_instruction(&decoded_instruction);
        }
    }

    fn load_instruction(self: &mut Self) -> &[u8] {
        let pc = self.regs[Register::PC as usize].0;
        self.regs[Register::PC as usize] += Wrapping(8);

        self.memory.read_instruction(pc)
    }

    fn execute_instruction(self: &mut Self, d: &DecodedInstruction) {
        let reg_1 = d.reg_1 as usize;
        let reg_2 = d.reg_2 as usize;
        let reg_3 = d.reg_3 as usize;

        match d.instruction_type {
            NOp => {}
            Halt => self.halt(),

            Increment => self.regs[reg_1] += Wrapping(1),
            Decrement => self.regs[reg_1] -= Wrapping(1),
            Add => self.regs[reg_1] = self.regs[reg_2] + self.regs[reg_3],
            Subtract => self.regs[reg_1] = self.regs[reg_2] - self.regs[reg_3],
            Multiply => self.regs[reg_1] = self.regs[reg_2] * self.regs[reg_3],
            Divide => self.regs[reg_1] = self.regs[reg_2] / self.regs[reg_3],
            Compare => {
                let l = self.regs[reg_1].0;
                let r = self.regs[reg_2].0;
                self.compare(l, r);
            }
            CompareImmediate => {
                let l = self.regs[reg_1].0;
                let r = d.operand;
                self.compare(l, r);
            }

            AddImmediate => self.regs[reg_1] = self.regs[reg_2] + Wrapping(d.operand),
            SubtractImmediate => self.regs[reg_1] = self.regs[reg_2] - Wrapping(d.operand),

            And => self.regs[reg_1] = self.regs[reg_2] & self.regs[reg_3],
            Or => self.regs[reg_1] = self.regs[reg_2] | self.regs[reg_3],
            XOr => self.regs[reg_1] = self.regs[reg_2] ^ self.regs[reg_3],
            Negate => self.regs[reg_1] = Wrapping((-i64::from(self.regs[reg_1].0)) as u32),
            Complement => self.regs[reg_1] = !self.regs[reg_1],

            LoadImmediate => self.regs[reg_1] = Wrapping(d.operand),
            Load => self.regs[reg_1] = Wrapping(self.memory.read_doubleword(self.regs[reg_2].0)),
            LoadByte => {
                self.regs[reg_1] = Wrapping(u32::from(self.memory.read(self.regs[reg_2].0)))
            }
            LoadDirect => self.regs[reg_1] = Wrapping(self.memory.read_doubleword(d.operand)),
            LoadDirectByte => self.regs[reg_1] = Wrapping(u32::from(self.memory.read(d.operand))),
            Store => self
                .memory
                .write_doubleword(self.regs[reg_2].0, self.regs[reg_1].0),
            StoreByte => self
                .memory
                .write(self.regs[reg_2].0, self.regs[reg_1].0 as u8),
            StoreDirect => self.memory.write_doubleword(d.operand, self.regs[reg_1].0),
            StoreDirectByte => self.memory.write(d.operand, self.regs[reg_1].0 as u8),
            Push => self.push(reg_1),
            Pop => self.pop(reg_1),

            Jump => self.regs[Register::PC as usize] = Wrapping(d.operand),
            Call => self.call(d.operand),
            Return => self.return_from_call(),

            BranchEqual => {
                if self.get_status_bit(StatusBit::Zero) {
                    self.regs[Register::PC as usize] = Wrapping(d.operand);
                }
            }

            BranchNotEqual => {
                if !self.get_status_bit(StatusBit::Zero) {
                    self.regs[Register::PC as usize] = Wrapping(d.operand);
                }
            }

            Move => self.regs[reg_1] = self.regs[reg_2],

            Invalid => panic!("Invalid instruction {:?}", d.instruction_type),
        }
    }

    fn push(&mut self, register: usize) {
        self.regs[Register::SP as usize] -= Wrapping(4);
        self.memory
            .write_doubleword(self.regs[Register::SP as usize].0, self.regs[register].0);
    }

    fn pop(&mut self, register: usize) {
        self.regs[register] = Wrapping(
            self.memory
                .read_doubleword(self.regs[Register::SP as usize].0),
        );
        self.regs[Register::SP as usize] += Wrapping(4);
    }

    fn call(&mut self, address: u32) {
        self.push(Register::PC as usize);
        self.regs[Register::PC as usize] = Wrapping(address);
    }

    fn return_from_call(&mut self) {
        self.pop(Register::PC as usize);
    }

    fn halt(self: &mut Self) {
        eprintln!("Halting CPU at PC=0x{:X}", self.get_register(Register::PC));
        self.halt = true;
    }

    fn compare(&mut self, l: u32, r: u32) {
        let left = Wrapping(l);
        let right = Wrapping(r);
        let result = left - right;

        self.set_status_bit(StatusBit::Zero, result.0 == 0);
        self.set_status_bit(StatusBit::Carry, right > left);

        self.set_status_bit(StatusBit::Negative, (result & Wrapping(1 << 31)).0 != 0);
    }
}

#[cfg(test)]
mod tests {
    use super::Register::*;
    use super::*;

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
        cpu.execute_instruction(&DecodedInstruction::new(LoadImmediate, 0, 0, 0, 1337));
        assert_eq!(cpu.get_register(R0), 1337);
    }

    #[test]
    fn test_increment() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(&DecodedInstruction::new(Increment, 1, 0, 0, 0));
        assert_eq!(cpu.get_register(R1), 11);
    }

    #[test]
    fn test_decrement() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(&DecodedInstruction::new(Decrement, 1, 0, 0, 0));
        assert_eq!(cpu.get_register(R1), 9);
    }

    #[test]
    fn test_add() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(&DecodedInstruction::new(Add, 0, 1, 2, 0));
        assert_eq!(cpu.get_register(R0), 15);
    }

    #[test]
    fn test_subtract() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(&DecodedInstruction::new(Subtract, 0, 1, 2, 0));
        assert_eq!(cpu.get_register(R0), 5);
    }

    #[test]
    fn test_multiply() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(&DecodedInstruction::new(Multiply, 0, 1, 2, 0));
        assert_eq!(cpu.get_register(R0), 50);
    }

    #[test]
    fn test_divide() {
        let mut cpu = cpu_arith_prep();
        cpu.execute_instruction(&DecodedInstruction::new(Divide, 0, 1, 2, 0));
        assert_eq!(cpu.get_register(R0), 2);
    }

    #[test]
    fn test_or() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(&DecodedInstruction::new(Or, 0, 1, 2, 0));
        assert_eq!(cpu.get_register(R0), 0b1111);
    }

    #[test]
    fn test_and() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(&DecodedInstruction::new(And, 0, 1, 2, 0));
        assert_eq!(cpu.get_register(R0), 0b1000);
    }

    #[test]
    fn test_xor() {
        let mut cpu = cpu_binary_prep();
        cpu.execute_instruction(&DecodedInstruction::new(XOr, 0, 1, 2, 0));
        assert_eq!(cpu.get_register(R0), 0b0111);
    }

    #[test]
    fn test_negate() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 0b00000000_00000001);
        cpu.execute_instruction(&DecodedInstruction::new(Negate, 0, 0, 0, 0));
        assert_eq!(cpu.get_register(R0), 0b11111111_11111111_11111111_11111111);
    }

    #[test]
    fn test_complement() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 0b00000000_00000001);
        cpu.execute_instruction(&DecodedInstruction::new(Complement, 0, 0, 0, 0));
        assert_eq!(cpu.get_register(R0), 0b11111111_11111111_11111111_11111110);
    }

    #[test]
    fn test_compare_equal() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 10);
        cpu.set_register(R1, 10);
        cpu.execute_instruction(&DecodedInstruction::new(Compare, 0, 1, 0, 0));
        assert!(cpu.get_status_bit(StatusBit::Zero));
        assert!(!cpu.get_status_bit(StatusBit::Carry));
        assert!(!cpu.get_status_bit(StatusBit::Negative));
    }

    #[test]
    fn test_compare_negative_carry() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 10);
        cpu.set_register(R1, 11);
        cpu.execute_instruction(&DecodedInstruction::new(Compare, 0, 1, 0, 0));
        assert!(!cpu.get_status_bit(StatusBit::Zero));
        assert!(cpu.get_status_bit(StatusBit::Carry));
        assert!(cpu.get_status_bit(StatusBit::Negative));
    }

    #[test]
    fn test_compare_negative() {
        let mut cpu = create_cpu();
        cpu.set_register(R0, 0xfffffff6); // -10
        cpu.set_register(R1, 11);
        cpu.execute_instruction(&DecodedInstruction::new(Compare, 0, 1, 0, 0));
        assert!(!cpu.get_status_bit(StatusBit::Zero));
        assert!(!cpu.get_status_bit(StatusBit::Carry));
        assert!(cpu.get_status_bit(StatusBit::Negative));
    }

    // TODO: Carry testcase?
    //    #[test]
    //    fn test_compare_carry() {
    //        let mut cpu = create_cpu();
    //        cpu.set_register(R0, 0x80000000); // -2147483648 most negative int
    //        cpu.set_register(R1, 5);
    //        cpu.execute_instruction(DecodedInstruction::new(
    //            Compare, 0, 1, 0, 0));
    //        assert!(!cpu.get_status_bit(StatusBit::Zero));
    //        assert!(cpu.get_status_bit(StatusBit::Carry));
    //        assert!(!cpu.get_status_bit(StatusBit::Negative));
    //    }
    #[test]
    fn test_branch_equal() {
        let mut cpu = create_cpu();
        cpu.execute_instruction(&DecodedInstruction::new(BranchEqual, 0, 0, 0, 0xCAFEBABE));
        assert_ne!(cpu.get_register(Register::PC), 0xCAFEBABE);
        cpu.set_status_bit(StatusBit::Zero, true);
        cpu.execute_instruction(&DecodedInstruction::new(BranchEqual, 0, 0, 0, 0xCAFEBABE));

        assert_eq!(cpu.get_register(Register::PC), 0xCAFEBABE);
    }

    #[test]
    fn test_branch_not_equal() {
        let mut cpu = create_cpu();
        cpu.set_status_bit(StatusBit::Zero, true);
        cpu.execute_instruction(&DecodedInstruction::new(
            BranchNotEqual,
            0,
            0,
            0,
            0xCAFEBABE,
        ));

        assert_ne!(cpu.get_register(Register::PC), 0xCAFEBABE);

        cpu.set_status_bit(StatusBit::Zero, false);
        cpu.execute_instruction(&DecodedInstruction::new(
            BranchNotEqual,
            0,
            0,
            0,
            0xCAFEBABE,
        ));
        assert_eq!(cpu.get_register(Register::PC), 0xCAFEBABE);
    }
}
