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
        todo!("implement thumb32 instructions");
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
                rd: (self.opcode.get(0..3)) as u8,
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
        *self & range.fold(0, |acc, bit| acc | (0b0000_0000_0000_0001 << bit))
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


