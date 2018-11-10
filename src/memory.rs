pub struct Memory {
    data: Vec<u32>
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: vec![0; 1024]
        }
    }

    fn read(self: &Self, addr: usize) -> u32 {
        if addr % 4 != 0 {
            panic!("Unaligned memory access at {:x}!", addr);
        }
        self.data[addr / 4]
    }

    fn write(self: &mut Self, addr: usize, value: u32) {
        if addr % 4 != 0 {
            panic!("Unaligned memory access at {:x}!", addr);
        }
        self.data[addr / 4] = value;
    }
}