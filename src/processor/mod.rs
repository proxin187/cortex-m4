pub mod instruction;
pub mod registers;
mod decoder;
mod fault;

use crate::bus::{DataBus, BitSize};
use crate::memory::Memory;

use instruction::{Instruction, InstructionKind};
use registers::Registers;
use decoder::Decoder;
use fault::{InterruptController, Exception};
use object::{File, Object, ObjectSection, SectionKind};

pub const RAM_CAPACITY: usize = 16380;
pub const FLASH_CAPACITY: usize = 65540;


#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Thread,
    Handle,
}

#[derive(Clone)]
pub struct Processor {
    flash: Memory,
    ram: Memory,
    nvic: InterruptController,
    pub mode: Mode,
    pub registers: Registers,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            flash: Memory::new(0x0, FLASH_CAPACITY),
            ram: Memory::new(0x20000000, RAM_CAPACITY),
            nvic: InterruptController::new(),
            mode: Mode::Thread,
            registers: Registers::new(),
        }
    }

    fn load_vtor(&mut self, handler_offset: usize) {
        let addr = self.registers.vtor.addr();
        let handler = self.read::<u32>(addr as usize + handler_offset);

        self.registers.sp.msp = self.read::<u32>(addr as usize) & 0xfffffffc;
        self.registers.sp.psp = 0;

        self.registers.set(15, |_| handler, self.mode);
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new();

        self.mode = Mode::Thread;

        self.load_vtor(4);
    }

    pub fn flash_data(&mut self, addr: usize, data: &[u8]) {
        for (offset, byte) in data.iter().enumerate() {
            self.write::<u8>(addr as usize + offset, *byte);
        }
    }

    pub fn flash(&mut self, rom: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::parse(rom)?;

        for section in file.sections() {
            match section.kind() {
                SectionKind::Text | SectionKind::Data => {
                    self.flash_data(section.address() as usize, section.data()?);
                },
                _ => {},
            }
        }

        Ok(())
    }

    pub fn fetch(&mut self) -> Instruction {
        match Decoder::new(self.read::<u16>(self.registers.get(15, self.mode) as usize - 4)) {
            Decoder::Thumb16(thumb16) => {
                Instruction {
                    kind: thumb16.decode(),
                    addr: self.registers.get(15, self.mode) - 4,
                    size: 2,
                }
            },
            Decoder::Thumb32(thumb32) => {
                Instruction {
                    kind: thumb32.decode(self.read::<u16>(self.registers.get(15, self.mode) as usize - 2)),
                    addr: self.registers.get(15, self.mode) - 8,
                    size: 4,
                }
            },
        }
    }

    fn execute(&mut self) {
        let inst = self.fetch();

        match inst.kind {
            InstructionKind::Mov { register, source } => {
                self.registers.set(register, |_| source.into(), self.mode);
            },
            InstructionKind::Add { rm, rn, rd } => {
                let result = self.registers.get(rm, self.mode) + self.registers.get(rn, self.mode);

                self.registers.set(rd, |_| result, self.mode);
            },
            InstructionKind::Blx { rm } => {
                self.nvic.throw(Exception::UsageFault);
            },
            InstructionKind::Bx { rm } => {
                let addr = self.registers.get(rm, self.mode);

                if addr & 0xf0000000 == 0xf0000000 {
                    self.exception_return(addr & !(0xf0000000));
                } else {
                    self.registers.set(15, |_| addr, self.mode);
                }
            },
            InstructionKind::B { imm11 } => {
                self.registers.set(15, |pc| (pc as i16 + imm11) as u32, self.mode);
            },
            InstructionKind::Ldr { rt, source } => {
                /*
                    base: 12
                    imm32: 4
                    address: 16
                    data: 536870928

                    base: 12
                    imm32: 8
                    address: 20
                    data: 536870912
                */

                let pc = 4 * (self.registers.get(15, self.mode) / 4);
                // let pc = 4 * (pc / 4);

                // println!("base: {}", pc);
                // println!("imm32: {:?}", source);

                /*
                for addr in 0..33 {
                    println!("{:x?}: {:x?}", addr, self.read::<u8>(addr));
                }
                */

                // TODO: maybe we are loading the program wrong as the addresses are valid in zmu
                // println!("address: {}", (pc + Into::<u32>::into(source)) as usize);

                // println!("{}", u16::from_le_bytes([0, 32]));

                let data = self.read::<u32>((pc + Into::<u32>::into(source)) as usize);

                // println!("data: {}", data);

                self.registers.set(rt, |_| data as u32, self.mode);
            },
            InstructionKind::LdrReg { rm, rn, rt } => {
                // TODO: implement ldr (register)
            },
            InstructionKind::LdrImm { source, rn, rt } => {
                let addr = self.registers.get(rn, self.mode) + Into::<u32>::into(source);

                let data = self.read::<u32>(addr as usize);

                self.registers.set(rt, |_| data, self.mode);
            },
            InstructionKind::Str { rt, rn } => {
                let value = self.registers.get(rt, self.mode);
                let addr = self.registers.get(rn, self.mode);

                // println!("addr: {}, value: {}", addr, value);

                self.write::<u32>(addr as usize, value);
            },
            InstructionKind::Undefined => panic!("undefined behaviour"),
        }

        self.registers.set(15, |pc| pc + inst.size, self.mode);
    }

    fn handle_exception(&mut self) {
        if let Some(exception) = self.nvic.poll() {
            match exception {
                Exception::Reset => self.reset(),
                _ => {
                    self.push_stack();

                    self.exception_entry(exception);
                },
            }
        }
    }

    pub fn step(&mut self) {
        self.execute();

        self.handle_exception();
    }
}

impl DataBus for Processor {
    fn read<T>(&mut self, addr: usize) -> T where T: BitSize + Default {
        match addr {
            0x0..0x10004 => self.flash.read(addr),
            0x20000000..0x20003ffc => self.ram.read(addr),
            0xe000ed08 => self.registers.vtor.read(),
            _ => { self.nvic.throw(Exception::BusFault); T::default() },
        }
    }

    fn write<T>(&mut self, addr: usize, value: T) where u32: From<T>, T: BitSize + Default + Into<u32> {
        match addr {
            0x0..0x10004 => self.flash.write(addr, value),
            0x20000000..0x20003ffc => self.ram.write(addr, value),
            0xe000ed08 => self.registers.vtor.write(value),
            _ => self.nvic.throw(Exception::BusFault),
        }
    }
}


