

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    Undefined,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::Mov { register, source } => f.write_fmt(format_args!("mov r<{}>, {}", register, Into::<u32>::into(source.clone()))),
            Instruction::Add { rm, rn, rd } => f.write_fmt(format_args!("add r<{}>, r<{}>, r<{}>", rd, rn, rm)),
            Instruction::Undefined => f.write_fmt(format_args!("undefined")),
        }
    }
}


