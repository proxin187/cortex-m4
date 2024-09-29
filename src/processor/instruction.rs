

#[derive(Debug, Clone, Copy)]
pub enum Source {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
}

impl Into<u32> for Source {
    fn into(self) -> u32 {
        match self {
            Source::Imm8(value) => value as u32,
            Source::Imm16(value) => value as u32,
            Source::Imm32(value) => value,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InstructionKind {
    Mov {
        register: u8,
        source: Source,
    },
    Add {
        rm: u8,
        rn: u8,
        rd: u8,
    },
    Blx {
        rm: u8,
    },
    Bx {
        rm: u8,
    },
    B {
        imm11: i16,
    },
    Ldr {
        rt: u8,
        source: Source,
    },
    LdrReg {
        rm: u8,
        rn: u8,
        rt: u8,
    },
    LdrImm {
        source: Source,
        rn: u8,
        rt: u8,
    },
    Str {
        rt: u8,
        rn: u8,
    },
    Undefined,
}

impl std::fmt::Display for InstructionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InstructionKind::Mov { register, source } => f.write_fmt(format_args!("mov r{}, {}", register, Into::<u32>::into(source.clone()))),
            InstructionKind::Add { rm, rn, rd } => f.write_fmt(format_args!("add r{}, r{}, r{}", rd, rn, rm)),
            InstructionKind::Blx { rm } => f.write_fmt(format_args!("blx r{}", rm)),
            InstructionKind::Bx { rm } => f.write_fmt(format_args!("bx r{}", rm)),
            InstructionKind::B { imm11 } => f.write_fmt(format_args!("b {}", imm11)),
            InstructionKind::Ldr { rt, source } => f.write_fmt(format_args!("ldr r{}, ={}", rt, Into::<u32>::into(source.clone()))),
            InstructionKind::LdrReg { rm, rn, rt } => f.write_fmt(format_args!("ldr r{}, [r{}, r{}]", rt, rn, rm)),
            InstructionKind::LdrImm { source, rn, rt } => f.write_fmt(format_args!("ldr r{}, [r{}, #{}]", rt, rn, Into::<u32>::into(source.clone()))),
            InstructionKind::Str { rt, rn } => f.write_fmt(format_args!("str r{}, [r{}]", rt, rn)),
            InstructionKind::Undefined => f.write_fmt(format_args!("undefined")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub addr: u32,
    pub size: u32,
}



