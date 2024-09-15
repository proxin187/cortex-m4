

pub struct Registers {
    registers: [u32; 16],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; 16],
        }
    }

    pub fn get_pc(&self) -> u32 { self.registers[15] }
}


