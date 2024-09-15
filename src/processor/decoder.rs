use std::ops::Range;

// https://web.eecs.umich.edu/~prabal/teaching/eecs373-f10/readings/ARMv7-M_ARM.pdf

// The Thumb instruction stream is a sequence of halfword-aligned halfwords. Each Thumb instruction is
// either a single 16-bit halfword in that stream, or a 32-bit instruction consisting of two consecutive halfwords
// in that stream.


pub struct Thumb32 {
    halfword: u16,
}

impl Thumb32 {
    pub fn new(halfword: u16) -> Thumb32 {
        Thumb32 {
            halfword,
        }
    }

    pub fn decode(&self, halfword: u16) {
    }
}

pub struct Thumb16 {
    halfword: u16,
}

impl Thumb16 {
    pub fn new(halfword: u16) -> Thumb16 {
        Thumb16 {
            halfword,
        }
    }

    pub fn decode(&self) {
    }
}

pub enum Decoder {
    Thumb16(Thumb16),
    Thumb32(Thumb32),
}

impl Decoder {
    pub fn new(halfword: u16) -> Decoder {
        match halfword.get(11..16) {
            0b11101 | 0b11110 |  0b11111 => Decoder::Thumb32(Thumb32::new(halfword)),
            _ => Decoder::Thumb16(Thumb16::new(halfword)),
        }
    }
}

pub trait BitVec {
    fn get(&self, range: Range<u8>) -> Self;
}

impl BitVec for u16 {
    fn get(&self, range: Range<u8>) -> Self {
        *self & range.fold(0, |acc, bit| acc | (0b1000_0000_0000_0000 >> bit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitvec() {
        let number: u16 = 0b1000_1111_0001_0111;

        println!("{:16b}", number);
        println!("{:16b}", number.get(0..16));
    }
}


