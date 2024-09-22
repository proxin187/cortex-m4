use crate::bus::{DataBus, BitSize};

use std::mem;


#[derive(Clone)]
pub struct Memory {
    start: usize,
    data: Vec<u8>,
}

impl Memory {
    pub fn new(start: usize, capacity: usize) -> Memory {
        Memory {
            start,
            data: vec![0; capacity],
        }
    }

    fn offset(&self, addr: usize) -> usize { addr - self.start }
}

impl DataBus for Memory {
    fn read<T>(&mut self, addr: usize) -> T where T: BitSize + Default {
        let offset = self.offset(addr);

        T::from(&self.data[offset..offset + mem::size_of::<T>()])
    }

    fn write<T>(&mut self, addr: usize, value: T) where  u32: From<T>, T: BitSize + Default + Into<u32> {
        let bytes = value.to_bytes();
        let offset = self.offset(addr);

        self.data[offset..offset + mem::size_of::<T>()].copy_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram() {
        let mut memory = Memory::new(0xff, 0xffff);

        memory.write::<u8>(0xff, 69);
        memory.write::<u16>(0xff1, 1337);
        memory.write::<u32>(0xff3, 69420);

        assert_eq!(memory.read::<u8>(0xff), 69);
        assert_eq!(memory.read::<u16>(0xff1), 1337);
        assert_eq!(memory.read::<u32>(0xff3), 69420);
    }
}


