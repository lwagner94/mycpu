use crate::common::util;
use crate::emulator::constants::*;
use crate::emulator::device::consoleio::ConsoleIO;
use crate::emulator::device::mainmemory::MainMemory;

pub trait Memory {
    fn read(&self, addr: u32) -> u8;
    fn read_doubleword(&self, addr: u32) -> u32;
    fn read_all(&self, addr: u32, number: u32) -> Vec<u8>;
    fn write(&mut self, addr: u32, value: u8);
    fn write_doubleword(&mut self, addr: u32, value: u32);
    fn write_all(&mut self, bytes: &[u8], offset: u32);

    fn read_instruction(&self, addr: u32) -> &[u8];

    fn size(&self) -> u32;
}

pub fn check_alignment(addr: u32, align: u32) {
    if addr % align != 0 {
        panic!("Unaligned memory access at {:x}", addr);
    }
}

pub fn address_to_index(addr: u32) -> usize {
    (addr as usize)
}

pub fn read_doubleword<F>(read_func: F, addr: u32) -> u32
where
    F: Fn(u32) -> u8,
{
    check_alignment(addr, 4);
    util::bytes_to_u32(
        read_func(addr),
        read_func(addr + 1),
        read_func(addr + 2),
        read_func(addr + 3),
    )
}

pub fn write_doubleword<F>(mut write_func: F, addr: u32, value: u32)
where
    F: FnMut(u32, u8),
{
    check_alignment(addr, 4);

    let bytes = util::u32_to_bytes(value);

    write_func(addr, bytes[0]);
    write_func(addr + 1, bytes[1]);
    write_func(addr + 2, bytes[2]);
    write_func(addr + 3, bytes[3]);
}

pub struct AddressSpace {
    memory: MainMemory,
    console: ConsoleIO,
}

impl Memory for AddressSpace {
    fn read(&self, addr: u32) -> u8 {
        let device = self.device_for_address(addr);
        device.read(addr)
    }

    fn read_doubleword(&self, addr: u32) -> u32 {
        let device = self.device_for_address(addr);
        device.read_doubleword(addr)
    }

    fn read_all(&self, addr: u32, number: u32) -> Vec<u8> {
        let device = self.device_for_address(addr);
        device.read_all(addr, number)
    }

    fn write(&mut self, addr: u32, value: u8) {
        let device = self.device_for_address_mut(addr);
        device.write(addr, value);
    }

    fn write_doubleword(&mut self, addr: u32, value: u32) {
        let device = self.device_for_address_mut(addr);
        device.write_doubleword(addr, value);
    }

    fn write_all(&mut self, bytes: &[u8], addr: u32) {
        let device = self.device_for_address_mut(addr);
        device.write_all(bytes, addr);
    }

    fn read_instruction(&self, addr: u32) -> &[u8] {
        self.memory.read_instruction(addr)
    }

    fn size(&self) -> u32 {
        // 4 Gigabyte
        0xFFFF_FFFF
    }
}

impl Default for AddressSpace {
    fn default() -> Self {
        AddressSpace {
            memory: MainMemory::new(MEMORY_START, MEMORY_SIZE),
            console: ConsoleIO::new(CONSOLEIO_START),
        }
    }
}

impl AddressSpace {
    fn device_for_address(&self, addr: u32) -> &Memory {
        match addr {
            MEMORY_START...MEMORY_END => &self.memory,
            CONSOLEIO_START...CONSOLEIO_END => &self.console,
            _ => panic!("Invalid memory access add 0x{:X}", addr),
        }
    }

    fn device_for_address_mut(&mut self, addr: u32) -> &mut Memory {
        match addr {
            MEMORY_START...MEMORY_END => &mut self.memory,
            CONSOLEIO_START...CONSOLEIO_END => &mut self.console,
            _ => panic!("Invalid memory access add 0x{:X}", addr),
        }
    }
}
