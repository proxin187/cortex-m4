use crate::bus::BitSize;

use std::mem;

use num::Num;


#[derive(Debug)]
pub enum Error {
    Checksum,
    Parse,
    Eof,
    Kind,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::Checksum => f.write_str("invalid checksum"),
            Error::Parse => f.write_str("failed to parse"),
            Error::Eof => f.write_str("reached eof"),
            Error::Kind => f.write_str("invalid record kind"),
        }
    }
}

impl std::error::Error for Error {}


#[derive(Debug)]
pub enum Kind {
    Data,
    Eof,
    ExtendSegmentAddress,
    StartSegmentAddress,
    ExtendLinearAddress,
    StartLinearAddress,
}

impl Kind {
    fn new(value: u8) -> Result<Kind, Box<dyn std::error::Error>> {
        match value {
            0 => Ok(Kind::Data),
            1 => Ok(Kind::Eof),
            2 => Ok(Kind::ExtendSegmentAddress),
            3 => Ok(Kind::StartSegmentAddress),
            4 => Ok(Kind::ExtendLinearAddress),
            5 => Ok(Kind::StartLinearAddress),
            _ => Err(Box::new(Error::Kind)),
        }
    }
}

#[derive(Debug)]
pub struct Record {
    pub addr: u16,
    pub kind: Kind,
    pub data: Vec<u8>,
    pub checksum: u8,
}

pub struct Hex<'a> {
    bytes: Box<dyn Iterator<Item = u8> + 'a>,
}

impl<'a> Hex<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Hex<'a>, Box<dyn std::error::Error>> {
        let iter = bytes.iter().map(|x| *x).filter(|x| x.is_ascii_hexdigit());

        Ok(Hex {
            bytes: Box::new(iter),
        })
    }

    fn take(&mut self, size: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer: Vec<u8> = Vec::new();

        while buffer.len() < size {
            let byte = self.bytes.next().ok_or(Box::new(Error::Eof))?;

            buffer.push(byte);
        }

        Ok(buffer)
    }

    fn take_str(&mut self, size: usize) -> Result<String, Box<dyn std::error::Error>> {
        String::from_utf8(self.take(size)?).map_err(|err| err.into())
    }

    fn read<T>(&mut self) -> Result<T, Box<dyn std::error::Error>> where T: BitSize + Num {
        let string = self.take_str(mem::size_of::<T>() * 2)?;

        Ok(T::from_str_radix(&string, 16).map_err(|_| Error::Parse)?)
    }

    fn read_vec<T>(&mut self, size: usize) -> Result<Vec<T>, Box<dyn std::error::Error>> where T: BitSize + Num {
        let mut buffer: Vec<T> = Vec::new();

        while buffer.len() < size {
            buffer.push(self.read::<T>()?);
        }

        Ok(buffer)
    }

    pub fn next(&mut self) -> Result<Record, Box<dyn std::error::Error>> {
        let size = self.read::<u8>()?;

        Ok(Record {
            addr: self.read::<u16>()?,
            kind: Kind::new(self.read::<u8>()?)?,
            data: self.read_vec(size as usize)?,
            checksum: self.read::<u8>()?,
        })
    }
}


