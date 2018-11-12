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

    fn address_to_index(&self, addr: u32) -> usize {
        (addr as usize)
    }

    fn check_alignment(&self, addr: u32, align: u32) {
        if addr % align != 0 {
            panic!("Unaligned memory access at {:x}", addr);
        }
    }

    pub fn read(&self, addr: u32) -> u8 {
        self.data[self.address_to_index(addr)]
    }

    pub fn read_instruction(&self, addr: u32) -> [u8; 8] {
        self.check_alignment(addr, 8);
        let index = self.address_to_index(addr);
        let mut res = [0; 8];
        res.copy_from_slice(&self.data[index..index + 8]);
        res
    }

    pub fn read_doubleword(&self, addr: u32) -> u32 {
        self.check_alignment(addr, 4);
        let b0 = self.read(addr) as u32;
        let b1 = self.read(addr + 1) as u32;
        let b2 = self.read(addr + 2) as u32;
        let b3 = self.read(addr + 3) as u32;

        b0 << 24 | b1 << 16 | b2 << 8 | b3
    }

    pub fn write(&mut self, addr: u32, value: u8) {
        let index = self.address_to_index(addr);
        self.data[index] = value;
    }

    pub fn write_all(&mut self, bytes: &[u8], offset: u32) {
        let index = self.address_to_index(offset);
        self.data[index..index + bytes.len()].copy_from_slice(bytes);
    }

    pub fn write_doubleword(&mut self, addr: u32, value: u32) {
        self.check_alignment(addr, 4);

        let bytes =
            [((0xFF_00_00_00 & value) >> 24) as u8,
            ((0x00_FF_00_00 & value) >> 16) as u8,
            ((0x00_00_FF_00 & value) >> 8) as u8,
            ((0x00_00_00_FF & value) >> 0) as u8];

        self.write_all(&bytes, addr);
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut mem = Memory::new(8);
        mem.write(7, 10);
        assert_eq!(mem.data, [0, 0, 0, 0, 0, 0, 0, 10]);
    }

    #[test]
    fn test_write_doubleword() {
        let mut mem = Memory::new(8);
        mem.write_doubleword(4, 0xAABBCCDD);
        assert_eq!(mem.data, [0, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD]);
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
            data: vec![0, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD]
        };
        assert_eq!(mem.read(4), 0xAA);
        assert_eq!(mem.read(5), 0xBB);
        assert_eq!(mem.read(6), 0xCC);
        assert_eq!(mem.read(7), 0xDD);
    }

    #[test]
    fn test_read_doubleword() {
        let mem = Memory {
            data: vec![0, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD]
        };
        assert_eq!(mem.read_doubleword(4), 0xAABBCCDD);
    }
}