struct Memory {
    data: Vec<u16>
}

impl Memory {
    fn read_word(self: &Self, addr: usize) -> u16 {
        if addr % 2 != 0 {
            panic!("Unaligned memory access at {:x}!", addr);
        }
        self.data[addr / 2]
    }

    fn write_word(self: &mut Self, addr: usize, value: u16) {
        if addr % 2 != 0 {
            panic!("Unaligned memory access at {:x}!", addr);
        }
        self.data[addr / 2] = value;
    }
}