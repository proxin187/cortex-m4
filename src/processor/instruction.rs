

#[derive(Debug)]
pub enum Source {
    Imm8(u8),
    Imm16(u16),
}

impl Into<u32> for Source {
    fn into(self) -> u32 {
        match self {
            Source::Imm8(value) => value as u32,
            Source::Imm16(value) => value as u32,
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mov {
        register: u8,
        source: Source,
    },
    Add {
        rm: u8,
        rn: u8,
        rd: u8,
    },
}


