use super::instruction::*;

use std::ops::Range;

// https://web.eecs.umich.edu/~prabal/teaching/eecs373-f10/readings/ARMv7-M_ARM.pdf


pub struct Thumb32 {
    halfword: u16,
}

impl Thumb32 {
    pub fn new(halfword: u16) -> Thumb32 {
        Thumb32 {
            halfword,
        }
    }

    pub fn decode(&self, halfword: u16) -> InstructionKind {
        InstructionKind::Undefined
    }
}

pub struct Thumb16 {
    opcode: u16,
}

impl Thumb16 {
    pub fn new(opcode: u16) -> Thumb16 {
        Thumb16 {
            opcode,
        }
    }

    pub fn decode(&self) -> InstructionKind {
        match self.opcode.get(11..16) {
            0b0010_0000_0000_0000 => InstructionKind::Mov {
                register: (self.opcode.get(8..11) >> 8) as u8,
                source: Source::Imm8((self.opcode.get(0..8)) as u8),
            },
            0b0001_1000_0000_0000 => InstructionKind::Add {
                rm: (self.opcode.get(6..9) >> 6) as u8,
                rn: (self.opcode.get(3..6) >> 3) as u8,
                rd: self.opcode.get(0..3) as u8,
            },
            0b0100_0000_0000_0000 => match self.opcode.get(7..16) {
                0b0100_0111_1000_0000 => InstructionKind::Blx {
                    rm: (self.opcode.get(3..7) >> 3) as u8,
                },
                0b0100_0111_0000_0000 => InstructionKind::Bx {
                    rm: (self.opcode.get(3..7) >> 3) as u8,
                },
                _ => InstructionKind::Undefined,
            },
            0b1110_0000_0000_0000 => InstructionKind::B {
                imm11: self.opcode.get(0..11).extend(11),
            },
            0b0100_1000_0000_0000 => InstructionKind::Ldr {
                rt: (self.opcode.get(8..11) >> 8) as u8,
                source: Source::Imm32((self.opcode.get(0..8) as u32) << 2),
            },
            0b0101_1000_0000_0000 => InstructionKind::LdrReg {
                rm: (self.opcode.get(6..9) >> 6) as u8,
                rn: (self.opcode.get(3..6) >> 3) as u8,
                rt: self.opcode.get(0..3) as u8,
            },
            0b0110_1000_0000_0000 => InstructionKind::LdrImm {
                source: Source::Imm8((self.opcode.get(6..11) >> 6) as u8),
                rn: (self.opcode.get(3..6) >> 3) as u8,
                rt: self.opcode.get(0..3) as u8,
            },
            0b0110_0000_0000_0000 => InstructionKind::Str {
                rt: self.opcode.get(0..3) as u8,
                rn: (self.opcode.get(3..6) >> 3) as u8,
            },
            _ => InstructionKind::Undefined,
        }
    }
}

pub enum Decoder {
    Thumb16(Thumb16),
    Thumb32(Thumb32),
}

impl Decoder {
    pub fn new(halfword: u16) -> Decoder {
        match halfword.get(11..16) {
            0b1110_1000_0000_0000 | 0b1111_0000_0000_0000 |  0b1111_1000_0000_0000 => Decoder::Thumb32(Thumb32::new(halfword)),
            _ => Decoder::Thumb16(Thumb16::new(halfword)),
        }
    }
}

pub trait BitVec {
    fn get(&self, range: Range<u8>) -> Self;
}

impl BitVec for u16 {
    fn get(&self, range: Range<u8>) -> Self {
        *self & range.fold(0, |acc, bit| acc | (1u16 << bit))
    }
}

impl BitVec for u32 {
    fn get(&self, range: Range<u8>) -> Self {
        *self & range.fold(0, |acc, bit| acc | (1u32 << bit))
    }
}

pub trait SignExtend<T> {
    fn extend(&self, topbit: i16) -> T;
}

impl SignExtend<i16> for u16 {
    fn extend(&self, topbit: i16) -> i16 {
        *self as i16 | ((1 << (16 - topbit)) - 1) << topbit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitvec() {
        let number: u16 = 0b1000_1111_0001_0111;

        println!("{:#018b}", number);
        println!("{:#018b}", number.get(0..12));
    }
}


