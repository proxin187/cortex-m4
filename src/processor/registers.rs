

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
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; 16],
            apsr: APSR::new(),
        }
    }

    pub fn mov<T>(&mut self, register: u8, imm: T) where T: Into<u32> {
        self.registers[register as usize] = imm.into();
    }

    pub fn add<T>(&mut self, register: u8, imm: T) where T: Into<u32> {
        self.registers[register as usize] += imm.into();
    }

    pub fn set(&mut self, register: u8, value: u32) {
        self.registers[register as usize] = value;
    }

    pub fn get(&self, register: u8) -> u32 { self.registers[register as usize] }

    pub fn all(&self) -> [u32; 16] { self.registers }
}


