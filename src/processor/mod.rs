pub mod instruction;
pub mod registers;
mod decoder;
mod fault;

use crate::bus::{DataBus, BitSize};
use crate::loader::{Hex, Kind};
use crate::memory::Memory;

use instruction::{Instruction, InstructionKind};
use registers::Registers;
use decoder::Decoder;

const RAM_CAPACITY: usize = 16380;
const FLASH_CAPACITY: usize = 65540;
const PRIVATE_PERIPHERAL_BUS_INTERNAL: usize = 0xe0040000 - 0xe0000000;
const PRIVATE_PERIPHERAL_BUS_EXTERNAL: usize = 0xe0100000 - 0xe0040000;


#[derive(Clone)]
pub struct Processor {
    flash: Memory,
    ram: Memory,
    ppbi: Memory,
    ppbe: Memory,
    pub registers: Registers,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            flash: Memory::new(0x0, FLASH_CAPACITY),
            ram: Memory::new(0x20000000, RAM_CAPACITY),
            ppbi: Memory::new(0xE0000000, PRIVATE_PERIPHERAL_BUS_INTERNAL),
            ppbe: Memory::new(0xE0040000, PRIVATE_PERIPHERAL_BUS_EXTERNAL),
            registers: Registers::new(),
        }
    }

    pub fn flash_data(&mut self, addr: u16, data: &[u8]) {
        for (offset, byte) in data.iter().enumerate() {
            self.flash.write::<u8>(addr as usize + offset, *byte);
        }
    }

    pub fn flash(&mut self, rom: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut hex = Hex::new(rom)?;

        loop {
            let record = hex.next()?;

            match record.kind {
                Kind::Data => {
                    self.flash_data(record.addr, &record.data);
                },
                Kind::ExtendSegmentAddress => {},
                Kind::StartSegmentAddress => {
                },
                Kind::ExtendLinearAddress => {},
                Kind::StartLinearAddress => {},
                Kind::Eof => break,
            }
        }

        Ok(())
    }

    pub fn fetch(&mut self) -> Instruction {
        match Decoder::new(self.read::<u16>(self.registers.get(15) as usize)) {
            Decoder::Thumb16(thumb16) => {
                self.registers.add(15, 2u32);

                Instruction {
                    kind: thumb16.decode(),
                    addr: self.registers.get(15) - 2,
                }
            },
            Decoder::Thumb32(thumb32) => {
                self.registers.add(15, 4u32);

                Instruction {
                    kind: thumb32.decode(self.read::<u16>(self.registers.get(15) as usize - 1)),
                    addr: self.registers.get(15) - 4,
                }
            },
        }
    }

    fn execute(&mut self) {
        let inst = self.fetch();

        match inst.kind {
            InstructionKind::Mov { register, source } => {
                self.registers.mov(register, source);
            },
            InstructionKind::Add { rm, rn, rd } => {
                self.registers.set(rd, self.registers.get(rm) + self.registers.get(rn));
            },
            InstructionKind::Undefined => panic!("undefined behaviour"),
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
            0xe0000000..0xe0040000 => self.ppbi.read(addr),
            0xe0040000..0xe0100000 => self.ppbe.read(addr),
            _ => panic!("out of bounds"),
        }
    }

    fn write<T>(&mut self, addr: usize, value: T) where T: BitSize {
        match addr {
            0x0..0x10004 => self.flash.write(addr, value),
            0x20000000..0x20003ffc => self.ram.write(addr, value),
            0xe0000000..0xe0040000 => self.ppbi.write(addr, value),
            0xe0040000..0xe0100000 => self.ppbe.write(addr, value),
            _ => panic!("out of bounds"),
        }
    }
}


