use super::Mode;


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
    pub fn new(msp: u32, psp: u32) -> StackPointer {
        StackPointer {
            msp,
            psp,
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
pub struct APSR {
    value: u32,
}

impl APSR {
    pub fn new() -> APSR {
        APSR {
            value: 0,
        }
    }

    pub fn get(&self, bit: u32) -> bool {
        ((self.value & (0b0000_0000_0000_0001 << bit)) >> bit) != 0
    }

    pub fn set(&mut self, bit: u32) {
        self.value |= 0b0000_0000_0000_0001 << bit;
    }

    pub fn unset(&mut self, bit: u32) {
        self.value &= !(0b0000_0000_0000_0001 << bit);
    }
}

#[derive(Clone)]
pub struct Registers {
    registers: [u32; 16],
    apsr: APSR,
    pub sp: StackPointer,
    pub control: Control,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; 16],
            apsr: APSR::new(),
            sp: StackPointer::new(0, 0),
            control: Control::new(false, false),
        }
    }

    pub fn set<F>(&mut self, register: u8, f: F, mode: Mode) where F: Fn(u32) -> u32 {
        match register {
            13 => self.sp.set(self.control, mode, |value| f(value)),
            _ => self.registers[register as usize] = f(self.registers[register as usize]),
        }
    }

    pub fn get(&self, register: u8, mode: Mode) -> u32 {
        match register {
            13 => self.sp.get(self.control, mode),
            _ => self.registers[register as usize],
        }
    }

    pub fn all(&self) -> [u32; 16] { self.registers }
}


