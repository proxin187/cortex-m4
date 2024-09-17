mod instruction;
mod registers;
mod decoder;

use crate::bus::{DataBus, BitSize};
use crate::memory::Memory;

use instruction::Instruction;
use registers::Registers;
use decoder::Decoder;

const RAM_CAPACITY: usize = 16380;
const FLASH_CAPACITY: usize = 65540;


#[derive(Clone)]
pub struct Processor {
    flash: Memory,
    ram: Memory,
    pub registers: Registers,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            flash: Memory::new(0x0, FLASH_CAPACITY),
            ram: Memory::new(0x20000000, RAM_CAPACITY),
            registers: Registers::new(),
        }
    }

    pub fn flash(&mut self, addr: u16, data: &[u8]) {
        for (offset, byte) in data.iter().enumerate() {
            self.flash.write::<u8>(addr as usize + offset, *byte);
        }
    }

    fn fetch(&mut self) -> Instruction {
        match Decoder::new(self.read::<u16>(self.registers.get(15) as usize)) {
            Decoder::Thumb16(thumb16) => {
                self.registers.add(15, 1u32);

                thumb16.decode()
            },
            Decoder::Thumb32(thumb32) => {
                self.registers.add(15, 2u32);

                thumb32.decode(self.read::<u16>(self.registers.get(15) as usize - 1))
            },
        }
    }

    fn execute(&mut self) {
        let inst = self.fetch();

        match inst {
            Instruction::Mov { register, source } => {
                self.registers.mov(register, source);
            },
            Instruction::Add { rm, rn, rd } => {
                self.registers.set(rd, self.registers.get(rm) + self.registers.get(rn));
            },
        }
    }

    pub fn step(&mut self) {
        self.execute();
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


