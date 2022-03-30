use crate::digit::*;

use std::{fmt, convert, /*, ops, cmp::{max, min}*/};

use num::BigUint;

#[derive(Clone)]
pub struct BigFixed {
    pub data: Vec<Digit>,
    pub position: isize
}

impl BigFixed {
    pub fn from_bytes_les(bytes: &[u8]) -> BigFixed {
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

    pub fn from_bytes_leu(bytes: &[u8]) -> BigFixed {
        let mut padded_bytes: Vec<u8> = Vec::with_capacity(bytes.len() + DIGITBYTES);
        padded_bytes.extend_from_slice(bytes);
        padded_bytes.resize(bytes.len() + DIGITBYTES, 0);
        BigFixed::from_bytes_les(&padded_bytes)
    }

    pub fn cast_unsigned(mut self) -> BigFixed {
        if self.is_neg() {
            self.data.push(0);
        }
        self
    }

    pub fn raw_get(&self, position: isize) -> Digit {
        let shifted = position - self.position;
        debug_assert!(shifted >= 0 && (shifted as usize) < self.data.len(), "bad position {}", position);
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
            (self.data[self.data.len() - 1] == 0 && self.data[self.data.len() - 2] < GREATESTBIT) ||
            (self.data[self.data.len() - 1] == ALLONES && self.data[self.data.len() - 2] >= GREATESTBIT)
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

    pub fn neg(&self) -> BigFixed {
        let mut data: Vec<Digit> = self.data.iter().map(|x| x ^ ALLONES).collect();
        let i = 0;
        let len = self.data.len();
        while i < len {
            data[i] = add(data[i], 1);
            if data[i] != 0 {break}
        }
        BigFixed {
            data: data,
            position: self.position
        }.trim()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.data.iter().flat_map(|x| x.to_le_bytes()).collect();
        bytes
    }
}

impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let neg = self.is_neg();
        let bytes = if neg {self.neg().to_bytes()} else {self.to_bytes()};
        let big = BigUint::from_bytes_le(&bytes);
        write!(f, "{}{}, {:?}", if neg {"-"} else {""}, big, self.data)
    }
}

impl convert::From<i8> for BigFixed {
    fn from(i: i8) -> BigFixed {
        if DIGITBYTES > 1 {
            BigFixed::from_bytes_les(&(i as SignedDigit).to_le_bytes())
        } else {
            BigFixed::from_bytes_les(&i.to_le_bytes())
        }
    }
}

impl convert::From<u8> for BigFixed {
    fn from(i: u8) -> BigFixed {
        if DIGITBYTES > 1 {
            BigFixed::from_bytes_leu(&(i as Digit).to_le_bytes())
        } else {
            BigFixed::from_bytes_leu(&i.to_le_bytes())
        }
    }
}

impl convert::From<i16> for BigFixed {
    fn from(i: i16) -> BigFixed {
        if DIGITBYTES > 2 {
            BigFixed::from_bytes_les(&(i as SignedDigit).to_le_bytes())
        } else {
            BigFixed::from_bytes_les(&i.to_le_bytes())
        }
    }
}

impl convert::From<u16> for BigFixed {
    fn from(i: u16) -> BigFixed {
        if DIGITBYTES > 2 {
            BigFixed::from_bytes_leu(&(i as Digit).to_le_bytes())
        } else {
            BigFixed::from_bytes_leu(&i.to_le_bytes())
        }
    }
}

impl convert::From<i32> for BigFixed {
    fn from(i: i32) -> BigFixed {
        if DIGITBYTES > 4 {
            BigFixed::from_bytes_les(&(i as SignedDigit).to_le_bytes())
        } else {
            BigFixed::from_bytes_les(&i.to_le_bytes())
        }
    }
}

impl convert::From<u32> for BigFixed {
    fn from(i: u32) -> BigFixed {
        if DIGITBYTES > 4 {
            BigFixed::from_bytes_leu(&(i as Digit).to_le_bytes())
        } else {
            BigFixed::from_bytes_leu(&i.to_le_bytes())
        }
    }
}

impl convert::From<i64> for BigFixed {
    fn from(i: i64) -> BigFixed {
        if DIGITBYTES > 8 {
            BigFixed::from_bytes_les(&(i as SignedDigit).to_le_bytes())
        } else {
            BigFixed::from_bytes_les(&i.to_le_bytes())
        }
    }
}

impl convert::From<u64> for BigFixed {
    fn from(i: u64) -> BigFixed {
        if DIGITBYTES > 8 {
            BigFixed::from_bytes_leu(&(i as Digit).to_le_bytes())
        } else {
            BigFixed::from_bytes_leu(&i.to_le_bytes())
        }
    }
}

impl convert::From<i128> for BigFixed {
    fn from(i: i128) -> BigFixed {
        if DIGITBYTES > 16 {
            BigFixed::from_bytes_les(&(i as SignedDigit).to_le_bytes())
        } else {
            BigFixed::from_bytes_les(&i.to_le_bytes())
        }
    }
}

impl convert::From<u128> for BigFixed {
    fn from(i: u128) -> BigFixed {
        if DIGITBYTES > 16 {
            BigFixed::from_bytes_leu(&(i as Digit).to_le_bytes())
        } else {
            BigFixed::from_bytes_leu(&i.to_le_bytes())
        }
    }
}
/*
impl ops::Add for BigFixed {
    type Output = BigFixed;
    fn add(self, other: BigFixed) -> BigFixed {
        &self + &other
    }
}

impl ops::Add for &BigFixed {
    type Output = BigFixed;
    fn add(self, other: &BigFixed) -> BigFixed {
        let low = min(self.data_low, other.data_low);
        let high = max(self.data_high, other.data_high);
        let width = high - low;
        // leave room for one overflow
        let mut v = vec![0 as u64; (width + 1) as usize];
        let mut carried: u128 = 0;
        let mut res: u128;
        for i in low..high {
            res = (self.get(i) as u128) + (other.get(i) as u128) + carried;
            carried = res >> 64;
            v[(i-low) as usize] = (res & O64D) as u64;
        }
        v[width as usize] = (carried & O64D) as u64;
        BigFixed {
            data: v,
            data_high: high,
            data_low: low
        }
    }
}*/
