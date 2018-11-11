pub struct Memory {
    data: Vec<u32>
}

impl Memory {
    pub fn new(size: u32) -> Self {
        Memory {
            data: vec![0; size as usize]
        }
    }

    fn address_to_index(self: &Self, addr: u32) -> usize {
        if addr % 4 != 0 {
            panic!("Unaligned memory access at {:x}!", addr);
        }
        (addr as usize) / 4
    }

    pub fn read(self: &Self, addr: u32) -> u32 {
        self.data[self.address_to_index(addr)]
    }

    pub fn read_instruction(self: &Self, addr: u32) -> [u32; 2] {
        let index = self.address_to_index(addr);
        let mut res = [0; 2];
        res.copy_from_slice(&self.data[index..index + 2]);
        res
    }

    pub fn write(self: &mut Self, addr: u32, value: u32) {
        let index = self.address_to_index(addr);
        self.data[index] = value;
    }

    pub fn write_instruction(self: &mut Self, addr: u32, instruction: [u32; 2]) {
        let index = self.address_to_index(addr);
        self.data[index..index + 2].copy_from_slice(&instruction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_addr_to_index_fail() {
        let mem = Memory::new(0);
        mem.address_to_index(3);
    }

    #[test]
    fn test_addr_to_index_pass() {
        let mem = Memory::new(0);
        assert_eq!(mem.address_to_index(0), 0);
        assert_eq!(mem.address_to_index(4), 1);
        assert_eq!(mem.address_to_index(16), 4);
    }

    #[test]
    fn test_write_instruction() {
        let mut mem = Memory::new(8);
        mem.write_instruction(8, [10, 20]);
        assert_eq!(mem.data, [0, 0, 10, 20, 0, 0, 0, 0]);
    }

    #[test]
    fn test_write() {
        let mut mem = Memory::new(8);
        mem.write(8, 10);
        assert_eq!(mem.data, [0, 0, 10, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_read_instruction() {
        let mem = Memory {
            data: vec![0, 0, 10, 20, 0, 0, 0, 0]
        };
        assert_eq!(mem.read_instruction(8), [10, 20]);
    }

    #[test]
    fn test_read() {
        let mem = Memory {
            data: vec![0, 0, 10, 20, 0, 0, 0, 0]
        };
        assert_eq!(mem.read(0), 0);
        assert_eq!(mem.read(4), 0);
        assert_eq!(mem.read(8), 10);
        assert_eq!(mem.read(12), 20);
    }
}