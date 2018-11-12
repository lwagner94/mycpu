use std::io::Write;
use std::io::Read;

pub struct Memory {
    data: Vec<u8>
}

impl Memory {
    pub fn new(size: u32) -> Self {
        Memory {
            data: vec![0; size as usize]
        }
    }

    fn address_to_index(self: &Self, addr: u32) -> usize {
        (addr as usize)
    }

    pub fn read(self: &Self, addr: u32) -> u8 {
        self.data[self.address_to_index(addr)]
    }

    pub fn read_instruction(self: &Self, addr: u32) -> [u8; 8] {
        let index = self.address_to_index(addr);
        let mut res = [0; 8];
        res.copy_from_slice(&self.data[index..index + 8]);
        res
    }

    pub fn write(self: &mut Self, addr: u32, value: u8) {
        let index = self.address_to_index(addr);
        self.data[index] = value;
    }

    pub fn write_instruction(self: &mut Self, addr: u32, instruction: [u8; 8]) {
        let index = self.address_to_index(addr);
        self.data[index..index + 8].copy_from_slice(&instruction);
    }

    pub fn write_all(&mut self, bytes: &[u8], offset: u32) {
        let index = self.address_to_index(offset);
        self.data[index..index + bytes.len()].copy_from_slice(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_instruction() {
        let mut mem = Memory::new(8);
        mem.write_instruction(0, [10, 20, 1, 2, 3, 4, 5, 6]);
        assert_eq!(mem.data, [10, 20, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_write() {
        let mut mem = Memory::new(8);
        mem.write(7, 10);
        assert_eq!(mem.data, [0, 0, 0, 0, 0, 0, 0, 10]);
    }

    #[test]
    fn test_read_instruction() {
        let mem = Memory {
            data: vec![0, 0, 10, 20, 0, 0, 0, 0]
        };
        assert_eq!(mem.read_instruction(0), [0, 0, 10, 20, 0, 0, 0, 0]);
    }

    #[test]
    fn test_read() {
        let mem = Memory {
            data: vec![0, 0, 10, 20, 0, 0, 0, 0]
        };
        assert_eq!(mem.read(0), 0);
        assert_eq!(mem.read(1), 0);
        assert_eq!(mem.read(2), 10);
        assert_eq!(mem.read(3), 20);
    }
}