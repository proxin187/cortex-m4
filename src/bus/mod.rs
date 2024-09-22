

pub trait BitSize {
    fn from(values: &[u8]) -> Self;

    fn to_bytes(&self) -> Vec<u8>;
}

impl BitSize for u8 {
    fn from(values: &[u8]) -> u8 { values[0] }

    fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}

impl BitSize for u16 {
    fn from(values: &[u8]) -> u16 { u16::from_le_bytes([values[0], values[1]]) }

    fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}

impl BitSize for u32 {
    fn from(values: &[u8]) -> u32 { u32::from_le_bytes([values[0], values[1], values[2], values[3]]) }

    fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}


pub trait DataBus {
    fn read<T>(&mut self, addr: usize) -> T where T: BitSize + Default;

    fn write<T>(&mut self, addr: usize, value: T) where u32: From<T>, T: BitSize + Default + Into<u32>;
}


