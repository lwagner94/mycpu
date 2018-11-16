use crate::emulator::device::mainmemory::MainMemory;
use crate::emulator::device::consoleio::ConsoleIO;

use crate::emulator::constants::*;

pub trait Memory {
    fn read(&self, addr: u32) -> u8;
    fn read_doubleword(&self, addr: u32) -> u32;
    fn read_all(&self, addr: u32, number: u32) -> Vec<u8>;
    fn write(&mut self, addr: u32, value: u8);
    fn write_doubleword(&mut self, addr: u32, value: u32);
    fn write_all(&mut self, bytes: &[u8], offset: u32);

    fn read_instruction(&self, addr: u32) -> [u8; 8];

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

pub fn read_doubleword<F>(read_func: F, addr: u32) -> u32 where F: Fn(u32) -> u8 {
    check_alignment(addr, 4);
    let b0 = read_func(addr) as u32;
    let b1 = read_func(addr + 1) as u32;
    let b2 = read_func(addr + 2) as u32;
    let b3 = read_func(addr + 3) as u32;

    b0 << 24 | b1 << 16 | b2 << 8 | b3
}

pub fn write_doubleword<F>(mut write_func: F, addr: u32, value: u32) where F: FnMut(u32, u8){
    check_alignment(addr, 4);

    write_func(addr, ((0xFF_00_00_00 & value) >> 24) as u8);
    write_func(addr + 1, ((0x00_FF_00_00 & value) >> 16) as u8);
    write_func(addr + 2, ((0x00_00_FF_00 & value) >> 8) as u8);
    write_func(addr + 3, ((0x00_00_00_FF & value) >> 0) as u8);
}

struct MappedDevice {
    start: u32,
    end: u32,
    device: Box<Memory>
}

pub struct AddressSpace {
    devices: Vec<MappedDevice>
}

impl Memory for AddressSpace {
    fn read(&self, addr: u32) -> u8 {
        let device = self.device_for_address(addr);
        device.device.read(addr - device.start)
    }

    fn read_doubleword(&self, addr: u32) -> u32 {
        let device = self.device_for_address(addr);
        device.device.read_doubleword(addr - device.start)
    }

    fn read_all(&self, addr: u32, number: u32) -> Vec<u8> {
        let device = self.device_for_address(addr);
        device.device.read_all(addr - device.start, number)
    }

    fn write(&mut self, addr: u32, value: u8) {
        let device = self.device_for_address_mut(addr);
        device.device.write(addr - device.start, value);
    }

    fn write_doubleword(&mut self, addr: u32, value: u32) {
        let device = self.device_for_address_mut(addr);
        device.device.write_doubleword(addr - device.start, value);
    }

    fn write_all(&mut self, bytes: &[u8], offset: u32) {
        let device = self.device_for_address_mut(offset);
        device.device.write_all(bytes, offset - device.start);
    }

    fn read_instruction(&self, addr: u32) -> [u8; 8] {
//        let device = self.device_for_address(addr);
        // Optimization
        let device = &self.devices[0];
        device.device.read_instruction(addr - device.start)
    }

    fn size(&self) -> u32 {
        // 4 Gigabyte
        0xFFFFFFFF
    }
}

impl AddressSpace {
    pub fn new() -> Self {
        AddressSpace{
            devices: Vec::new()
        }
    }

    pub fn default() -> Self {
        let mut mem = AddressSpace::new();

        let main_memory = Box::new(MainMemory::new(1024 * 1024));

        // Map main memory at 1 Megabyte
        // Memory space: 0x100000 - 0x1fffff
        mem.map(main_memory, MEMORY_START);

        // ConsoleIO: 8 bytes
        let console_io = Box::new(ConsoleIO::new());

        // Map at 512kByte
        // Map it at 0x80000 - 0x80007
        mem.map(console_io, 0x80000);

        mem
    }

    pub fn map(&mut self, device: Box<Memory>, offset: u32) {
        let size = device.size();

        let start = offset;
        let end = start + size - 1; // Inclusive

        self.devices.push(MappedDevice {
            start,
            end,
            device
        });
    }

    fn device_for_address_mut(&mut self, addr: u32) -> &mut MappedDevice {
        // TODO: Find out how to write this in idiomatic Rust code

        let mut index = 0usize;

        for i in 0..self.devices.len() {
            if addr >= self.devices[i].start && addr <= self.devices[i].end {
                index = i;
            }
        }

        &mut self.devices[index]
    }

    fn device_for_address(&self, addr: u32) -> &MappedDevice {
        // TODO: Find out how to write this in idiomatic Rust code

        let mut index = 0usize;

        for i in 0..self.devices.len() {
            if addr >= self.devices[i].start && addr <= self.devices[i].end {
                index = i;
            }
        }

        &self.devices[index]
    }
}