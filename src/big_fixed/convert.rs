/*use crate::{digit::*, BigFixed};

use std::{convert, fmt};

use num_bigint::{Sign, BigInt};

impl BigFixed {
    pub fn int_from_bytes(bytes: &[u8], position: isize, unsigned: bool) -> BigFixed {
        let good_len = bytes.len() / DIGITBYTES;
        let good_bytes_len = good_len * DIGITBYTES;
        let mut data: Vec<Digit> = Vec::with_capacity(good_len + 2);
        data.extend(
            (0..good_len).map(
                |i| i * DIGITBYTES
            ).map(
                |j| digit_from_bytes(&bytes[j..(j+DIGITBYTES)])
            )
        );
        if bytes.len() != good_bytes_len {
            let fill: u8 = if !unsigned && (bytes.len() - good_bytes_len) > 0 && bytes[bytes.len()-1] >= 128 {255} else {0};
            let mut int_bytes = [fill; DIGITBYTES];
            // isn't there a function for this?
            for i in 0..(bytes.len() - good_bytes_len) {
                int_bytes[i] = bytes[good_bytes_len + i];
            }
            data.push(Digit::from_le_bytes(int_bytes));
        }
        BigFixed::construct(data, position, unsigned)
    }
}

macro_rules! from_signed {
    ($s: ty, $n: literal) => {
        impl convert::From<$s> for BigFixed {
            fn from(i: $s) -> BigFixed {
                if DIGITBYTES > $n {
                    BigFixed::from_bytes(&(i as SignedDigit).to_le_bytes() as &[u8], 0, false)
                } else {
                    BigFixed::from_bytes(&i.to_le_bytes() as &[u8], 0, false)
                }
            }
        }
    };
}

macro_rules! from_unsigned {
    ($u: ty, $n: literal) => {
        impl convert::From<$u> for BigFixed {
            fn from(u: $u) -> BigFixed {
                if DIGITBYTES > $n {
                    BigFixed::from_bytes(&(u as SignedDigit).to_le_bytes() as &[u8], 0, true)
                } else {
                    BigFixed::from_bytes(&u.to_le_bytes() as &[u8], 0, true)
                }
            }
        }
    };
}

from_signed!(i8, 1);
from_unsigned!(u8, 1);
from_signed!(i16, 2);
from_unsigned!(u16, 2);
from_signed!(i32, 4);
from_unsigned!(u32, 4);
from_signed!(i64, 8);
from_unsigned!(u64, 8);
from_signed!(i128, 16);
from_unsigned!(u128, 16);

// data to BigInt
impl convert::From<&BigFixed> for BigInt {
    fn from(x: &BigFixed) -> BigInt {
        BigInt::from_bytes_le(
            if x.is_neg() {Sign::Minus} else {Sign::Plus},
            &x.abs().data_to_bytes()
        )
    }
}

impl BigFixed {
    pub fn int_bigint(&self) -> BigInt {
        if self.position < 0 {
            self.int().int_bigint()
        } else {
            BigInt::from(self) << (SINGLEWIDE * self.position as usize)
        }
    }
}*/
