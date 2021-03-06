use crate::emulator::memory::{
    address_to_index, check_alignment, read_doubleword, write_doubleword, Memory,
};

pub struct MainMemory {
    data: Vec<u8>,
    offset: u32,
}

impl Memory for MainMemory {
    fn read(&self, addr: u32) -> u8 {
        self.data[address_to_index(addr - self.offset)]
    }

    fn read_doubleword(&self, addr: u32) -> u32 {
        read_doubleword(|byte_addr| self.read(byte_addr), addr)
    }

    fn read_all(&self, addr: u32, number: u32) -> Vec<u8> {
        check_alignment(addr, 4);
        let index = address_to_index(addr - self.offset);
        let mut res = vec![0u8; number as usize];
        res.copy_from_slice(&self.data[index..index + 8]);
        res
    }

    fn write(&mut self, addr: u32, value: u8) {
        let index = address_to_index(addr - self.offset);
        self.data[index] = value;
    }

    fn write_doubleword(&mut self, addr: u32, value: u32) {
        // No offset needed, because write() applies it as well
        write_doubleword(|byte_addr, byte| self.write(byte_addr, byte), addr, value);
    }

    fn write_all(&mut self, bytes: &[u8], addr: u32) {
        let index = address_to_index(addr - self.offset);
        self.data[index..index + bytes.len()].copy_from_slice(bytes);
    }

    fn read_instruction(&self, addr: u32) -> &[u8] {
        check_alignment(addr, 8);
        let index = address_to_index(addr - self.offset);
        &self.data[index..index + 8]
    }

    fn size(&self) -> u32 {
        self.data.len() as u32
    }
}

impl MainMemory {
    pub fn new(offset: u32, size: u32) -> Self {
        MainMemory {
            offset,
            data: vec![0; size as usize],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut mem = MainMemory::new(0, 8);
        mem.write(7, 10);
        assert_eq!(mem.data, [0, 0, 0, 0, 0, 0, 0, 10]);
    }

    #[test]
    fn test_write_doubleword() {
        let mut mem = MainMemory::new(0, 8);
        mem.write_doubleword(4, 0xAABBCCDD);
        assert_eq!(mem.data, [0, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD]);
    }

    #[test]
    fn test_read_instruction() {
        let mem = MainMemory {
            offset: 0,
            data: vec![0, 0, 10, 20, 0, 0, 0, 0],
        };
        assert_eq!(mem.read_all(0, 8), vec![0, 0, 10, 20, 0, 0, 0, 0]);
    }

    #[test]
    fn test_read() {
        let mem = MainMemory {
            offset: 0,
            data: vec![0, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD],
        };
        assert_eq!(mem.read(4), 0xAA);
        assert_eq!(mem.read(5), 0xBB);
        assert_eq!(mem.read(6), 0xCC);
        assert_eq!(mem.read(7), 0xDD);
    }

    #[test]
    fn test_read_doubleword() {
        let mem = MainMemory {
            offset: 0,
            data: vec![0, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD],
        };
        assert_eq!(mem.read_doubleword(4), 0xAABBCCDD);
    }
}
