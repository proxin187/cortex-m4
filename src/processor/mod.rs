mod instruction;
mod registers;
mod decoder;

use crate::bus::{DataBus, BitSize};
use crate::memory::Memory;

use registers::Registers;
use decoder::Decoder;

use std::mem;

const RAM_CAPACITY: usize = 16380;
const FLASH_CAPACITY: usize = 65540;


pub struct Processor {
    flash: Memory,
    ram: Memory,
    registers: Registers,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            flash: Memory::new(0x0, FLASH_CAPACITY),
            ram: Memory::new(0x20000000, RAM_CAPACITY),
            registers: Registers::new(),
        }
    }

    fn fetch(&mut self) {
        let inst = self.read::<u16>(self.registers.get_pc() as usize);
    }

    pub fn flash(&mut self, addr: u16, data: &[u8]) {
        for (offset, byte) in data.iter().enumerate() {
            self.flash.write::<u8>(addr as usize + offset, *byte);
        }
    }

    pub fn step(&mut self) {
    }
}

impl DataBus for Processor {
    fn read<T>(&self, addr: usize) -> T where T: BitSize {
        match addr {
            0x0..0x10004 => self.flash.read(addr),
            0x20000000..0x20003ffc => self.ram.read(addr),
            _ => panic!("out of bounds"),
        }
    }

    fn write<T>(&mut self, addr: usize, value: T) where T: BitSize {
        match addr {
            0x0..0x10004 => self.flash.write(addr, value),
            0x20000000..0x20003ffc => self.ram.write(addr, value),
            _ => panic!("out of bounds"),
        }
    }
}


