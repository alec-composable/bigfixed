use crate::digit::*;

use std::{fmt, convert, cmp::{max, min}};

use num_bigint::{BigInt, Sign};

#[derive(Clone)]
pub struct BigFixed {
    pub data: Vec<Digit>,
    pub position: isize
}

// static functions
impl BigFixed {
    pub fn unsign_byte_stream(bytes: &[u8]) -> Vec<u8> {
        let mut padded_bytes: Vec<u8> = Vec::with_capacity(bytes.len() + DIGITBYTES);
        padded_bytes.extend_from_slice(bytes);
        padded_bytes.resize(bytes.len() + DIGITBYTES, 0);
        padded_bytes
    }
}

impl BigFixed {
    pub fn raw_get(&self, position: isize) -> Digit {
        let shifted = position - self.position;
        debug_assert!(shifted >= 0 && (shifted as usize) < self.data.len(), "bad position: {}", position);
        self.data[shifted as usize]
    }

    pub fn greatest_digit(&self) -> Digit {
        if self.data.len() > 0 {
            self.data[self.data.len() - 1]
        } else {
            0
        }
    }

    pub fn trim(mut self) -> BigFixed {
        while self.data.len() > 1 && (
            (self.data[self.data.len()-1] == 0       && self.data[self.data.len()-2] <  GREATESTBIT) ||
            (self.data[self.data.len()-1] == ALLONES && self.data[self.data.len()-2] >= GREATESTBIT)
        ) {
            self.data.pop();
        }
        while self.data.len() > 0 && self.data[0] == 0 {
            self.data.remove(0);
            self.position = self.position + 1;
        }
        if self.data.len() == 0 {
            self.position = 0;
        }
        self
    }

    pub fn is_neg(&self) -> bool {
        self.greatest_digit() >= GREATESTBIT
    }

    pub fn cast_unsigned(mut self) -> BigFixed {
        if self.is_neg() {
            self.data.push(0);
        }
        self
    }

    pub fn neg(&self) -> BigFixed {
        let mut data: Vec<Digit> = self.data.iter().map(
            |x| x ^ ALLONES
        ).collect();
        let i = 0;
        while i < self.data.len() {
            data[i] = add(data[i], 1);
            if data[i] != 0 {break}
        }
        BigFixed {
            data: data,
            position: self.position
        }.trim()
    }

    pub fn to_bytes_data(&self) -> Vec<u8> {
        self.data.iter().flat_map(|x| x.to_le_bytes()).collect()
    }

    pub fn shift(&mut self, shift: isize) -> &BigFixed{
        self.position = self.position + shift;
        self
    }

    pub fn data_to_bigint(&self) -> BigInt {
        if self.is_neg() {
            BigInt::from_bytes_le(Sign::Minus, &self.neg().to_bytes_data())
        } else {
            BigInt::from_bytes_le(Sign::Plus, &self.to_bytes_data())
        }
    }

    pub fn floor(&self) -> BigFixed {
        let mut data = self.data.clone();
        if self.position < 0 {
            data.drain(0..(-self.position) as usize);
        }
        BigFixed
    }

    pub fn frac(&self) -> BigInt {
        let frac = &self.data[0..max(0, -self.position) as usize];
        let bytes: Vec<u8> = frac.iter().flat_map(|i| i.to_le_bytes()).collect();
        BigInt::from_bytes_le(Sign::Plus, &bytes)
    }
}

impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("displaying {:?} {}", self.data, self.position);
        let big = BigInt::from(self);
        println!("big {}", big);
        if self.position >= 0 {
            write!(f, "{}", big)
        } else {
            write!(f, "{}.{}", big, self.frac())
        }
    }
}

// little endian unsigned integer
impl convert::From<&[u8]> for BigFixed {
    fn from(bytes: &[u8]) -> BigFixed {
        assert_eq!(bytes.len() % DIGITBYTES, 0, "byte stream must be a multiple of {}", DIGITBYTES);

        // number of positions required
        let len: usize = bytes.len() / DIGITBYTES;
        
        let mut data: Vec<Digit> = Vec::with_capacity(len);
        data.extend(
            (0..len).map(
                |i| i * DIGITBYTES
            ).map(
                |j| digit_from_bytes(&bytes[j..(j+DIGITBYTES)])
            )
        );
        BigFixed {
            data: data,
            position: 0
        }.trim()
    }
}

impl convert::From<i8> for BigFixed {
    fn from(i: i8) -> BigFixed {
        if DIGITBYTES > 1 {
            BigFixed::from(&(i as SignedDigit).to_le_bytes() as &[u8])
        } else {
            BigFixed::from(&i.to_le_bytes() as &[u8])
        }
    }
}

impl convert::From<u8> for BigFixed {
    fn from(i: u8) -> BigFixed {
        if DIGITBYTES > 1 {
            BigFixed::from(BigFixed::unsign_byte_stream(&(i as Digit).to_le_bytes()).as_slice())
        } else {
            BigFixed::from(BigFixed::unsign_byte_stream(&i.to_le_bytes()).as_slice())
        }
    }
}

impl convert::From<i16> for BigFixed {
    fn from(i: i16) -> BigFixed {
        if DIGITBYTES > 2 {
            BigFixed::from(&(i as SignedDigit).to_le_bytes() as &[u8])
        } else {
            BigFixed::from(&i.to_le_bytes() as &[u8])
        }
    }
}

impl convert::From<u16> for BigFixed {
    fn from(i: u16) -> BigFixed {
        if DIGITBYTES > 2 {
            BigFixed::from(BigFixed::unsign_byte_stream(&(i as Digit).to_le_bytes()).as_slice())
        } else {
            BigFixed::from(BigFixed::unsign_byte_stream(&i.to_le_bytes()).as_slice())
        }
    }
}

impl convert::From<i32> for BigFixed {
    fn from(i: i32) -> BigFixed {
        if DIGITBYTES > 4 {
            BigFixed::from(&(i as SignedDigit).to_le_bytes() as &[u8])
        } else {
            BigFixed::from(&i.to_le_bytes() as &[u8])
        }
    }
}

impl convert::From<u32> for BigFixed {
    fn from(i: u32) -> BigFixed {
        if DIGITBYTES > 4 {
            BigFixed::from(BigFixed::unsign_byte_stream(&(i as Digit).to_le_bytes()).as_slice())
        } else {
            BigFixed::from(BigFixed::unsign_byte_stream(&i.to_le_bytes()).as_slice())
        }
    }
}

impl convert::From<i64> for BigFixed {
    fn from(i: i64) -> BigFixed {
        if DIGITBYTES > 8 {
            BigFixed::from(&(i as SignedDigit).to_le_bytes() as &[u8])
        } else {
            BigFixed::from(&i.to_le_bytes() as &[u8])
        }
    }
}

impl convert::From<u64> for BigFixed {
    fn from(i: u64) -> BigFixed {
        if DIGITBYTES > 8 {
            BigFixed::from(BigFixed::unsign_byte_stream(&(i as Digit).to_le_bytes()).as_slice())
        } else {
            BigFixed::from(BigFixed::unsign_byte_stream(&i.to_le_bytes()).as_slice())
        }
    }
}

impl convert::From<i128> for BigFixed {
    fn from(i: i128) -> BigFixed {
        if DIGITBYTES > 16 {
            BigFixed::from(&(i as SignedDigit).to_le_bytes() as &[u8])
        } else {
            BigFixed::from(&i.to_le_bytes() as &[u8])
        }
    }
}

impl convert::From<u128> for BigFixed {
    fn from(i: u128) -> BigFixed {
        if DIGITBYTES > 16 {
            BigFixed::from(BigFixed::unsign_byte_stream(&(i as Digit).to_le_bytes()).as_slice())
        } else {
            BigFixed::from(BigFixed::unsign_byte_stream(&i.to_le_bytes()).as_slice())
        }
    }
}

// floor
impl convert::From<&BigFixed> for BigInt {
    fn from(x: &BigFixed) -> BigInt {
        let neg = x.is_neg();
        let sign = if neg {Sign::Minus} else {Sign::Plus};
        let mut pos_bytes;
        if neg {
            pos_bytes = x.neg().to_bytes_data();
        } else {
            pos_bytes = x.to_bytes_data();
        }
        println!("position {}", x.position);
        if x.position >= 0 {
            pos_bytes.splice(0..0, (0..(x.position as usize * DIGITBYTES)).map(|_| 0));
        } else {
            pos_bytes.drain(0..min(pos_bytes.len(), (-x.position) as usize * DIGITBYTES));            
        }
        BigInt::from_bytes_le(sign, &pos_bytes)
    }
}
