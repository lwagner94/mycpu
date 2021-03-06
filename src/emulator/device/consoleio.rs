use crate::emulator::memory::*;

#[derive(Default)]
pub struct ConsoleIO {
    offset: u32,
}

impl Memory for ConsoleIO {
    fn read(&self, _addr: u32) -> u8 {
        unimplemented!();
    }

    fn read_doubleword(&self, _addr: u32) -> u32 {
        unimplemented!();
    }

    fn read_all(&self, addr: u32, number: u32) -> Vec<u8> {
        check_alignment(addr, 4);
        let res = vec![0u8; number as usize];
        res
    }

    fn write(&mut self, addr: u32, value: u8) {
        match addr - self.offset {
            0 => print!("{}", value as char),
            _ => panic!("ConsoleIO: Invalid memory access at offset 0x{:X}", addr),
        }
    }

    fn write_doubleword(&mut self, addr: u32, value: u32) {
        let byte = ((value & 0xFF_00_00_00) >> 24) as u8;
        self.write(addr - self.offset, byte);
    }

    fn write_all(&mut self, _bytes: &[u8], _offset: u32) {
        unimplemented!();
    }

    fn read_instruction(&self, _addr: u32) -> &[u8] {
        unimplemented!()
    }

    fn size(&self) -> u32 {
        8
    }
}

impl ConsoleIO {
    pub fn new(offset: u32) -> Self {
        ConsoleIO { offset }
    }
}
