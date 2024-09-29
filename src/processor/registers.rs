use super::decoder::BitVec;
use super::{Mode, RAM_CAPACITY};

use crate::bus::BitSize;


// page 429 @ ARMv7M Reference Manual
#[derive(Clone, Copy)]
pub struct Control {
    pub private: bool,
    pub stack: bool,
}

impl Control {
    pub fn new(private: bool, stack: bool) -> Control {
        Control {
            private,
            stack,
        }
    }
}

#[derive(Clone)]
pub struct StackPointer {
    pub msp: u32,
    pub psp: u32,
}

impl StackPointer {
    pub fn new(sp: u32) -> StackPointer {
        StackPointer {
            msp: sp,
            psp: sp,
        }
    }

    pub fn set<F>(&mut self, control: Control, mode: Mode, f: F) where F: Fn(u32) -> u32 {
        if control.stack && mode == Mode::Thread {
            self.psp = f(self.psp);
        } else {
            self.msp = f(self.msp);
        }
    }

    pub fn get(&self, control: Control, mode: Mode) -> u32 {
        (control.stack && mode == Mode::Thread).then(|| self.psp).unwrap_or(self.msp)
    }
}

#[derive(Clone)]
pub struct PSR {
    pub value: u32,
}

impl PSR {
    pub fn new() -> PSR {
        PSR {
            value: 0,
        }
    }

    pub fn get(&self, bit: u32) -> bool {
        ((self.value & (1 << bit)) >> bit) != 0
    }

    pub fn set(&mut self, bit: u32) {
        self.value |= 1 << bit;
    }

    pub fn unset(&mut self, bit: u32) {
        self.value &= !(1 << bit);
    }
}

pub enum TableBase {
    Code,
    Ram,
}

impl TableBase {
    fn value(self) -> u32 {
        match self {
            TableBase::Code => 0,
            TableBase::Ram => 1,
        }
    }
}

#[derive(Clone)]
pub struct Vtor {
    value: u32,
}

impl Vtor {
    pub fn new(base: TableBase, offset: u32) -> Vtor {
        Vtor {
            value: (base.value() << 29) | offset << 7,
        }
    }

    pub fn read<T>(&self) -> T where T: BitSize { T::from(&self.value.to_bytes()) }

    pub fn write<T>(&mut self, value: T) where T: BitSize + Into<u32> { self.value = value.into() }

    pub fn addr(&self) -> u32 {
        match self.value & (1 << 29) {
            0 => self.value.get(7..29) >> 7,
            _ => (self.value.get(7..29) >> 7) + 0x20000000,
        }
    }
}

#[derive(Clone)]
pub struct Registers {
    registers: [u32; 16],
    pub vtor: Vtor,
    pub psr: PSR,
    pub sp: StackPointer,
    pub control: Control,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; 16],
            vtor: Vtor::new(TableBase::Code, 0),
            psr: PSR::new(),
            sp: StackPointer::new((0x20000000 + RAM_CAPACITY) as u32),
            control: Control::new(false, false),
        }
    }

    pub fn set<F>(&mut self, register: u8, f: F, mode: Mode) where F: Fn(u32) -> u32 {
        match register {
            13 => self.sp.set(self.control, mode, |value| f(value)),
            15 => self.registers[register as usize] = f(self.registers[register as usize]) & 0xfffffffe,
            _ => self.registers[register as usize] = f(self.registers[register as usize]),
        }
    }

    pub fn get(&self, register: u8, mode: Mode) -> u32 {
        match register {
            13 => self.sp.get(self.control, mode),
            15 => self.registers[register as usize] + 4,
            _ => self.registers[register as usize],
        }
    }

    pub fn all(&self) -> [u32; 16] { self.registers }
}


