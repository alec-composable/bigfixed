use crate::{digit::*, Tail, BigFixed};

use std::{convert};

//use num_bigint::{Sign, BigInt};

impl BigFixed {
    // little endian bytes
    pub fn int_from_bytes(bytes: &[u8], unsigned: bool) -> BigFixed {
        // extension stuff in case bytes length does not divide DIGITBYTES evenly
        let good_len = bytes.len() / DIGITBYTES;
        let good_bytes_len = good_len * DIGITBYTES;
        // one extra for byte-extended digit
        let mut data: Vec<Digit> = Vec::with_capacity(good_len + 1);
        data.extend(
            (0..good_len).map(
                |i| i * DIGITBYTES
            ).map(
                |j| digit_from_bytes(&bytes[j..(j+DIGITBYTES)])
            )
        );
        if bytes.len() != good_bytes_len {
            // have to extend the greatest digit's bytes
            let fill: u8 = if !unsigned && bytes[bytes.len()-1] >= 128 {255} else {0};
            let mut int_bytes = [fill; DIGITBYTES];
            // isn't there an idiomatic way to do this?
            for i in 0..(bytes.len() - good_bytes_len) {
                int_bytes[i] = bytes[good_bytes_len + i];
            }
            data.push(digit_from_bytes(&int_bytes));
        }
        let is_neg = !unsigned && data.len() > 0 && data[data.len()-1] >= GREATESTBIT;
        BigFixed::construct(
            if is_neg {ALLONES} else {0},
            data,
            Tail::from(vec![0]),
            0
        )
    }
}

macro_rules! from_signed_int {
    ($s: ty, $n: literal) => {
        impl convert::From<$s> for BigFixed {
            fn from(i: $s) -> BigFixed {
                if DIGITBYTES > $n {
                    BigFixed::int_from_bytes(&(i as SignedDigit).to_le_bytes() as &[u8], false)
                } else {
                    BigFixed::int_from_bytes(&i.to_le_bytes() as &[u8], false)
                }
            }
        }
    };
}

macro_rules! from_unsigned_int {
    ($u: ty, $n: literal) => {
        impl convert::From<$u> for BigFixed {
            fn from(u: $u) -> BigFixed {
                if DIGITBYTES > $n {
                    BigFixed::int_from_bytes(&(u as SignedDigit).to_le_bytes() as &[u8], true)
                } else {
                    BigFixed::int_from_bytes(&u.to_le_bytes() as &[u8], true)
                }
            }
        }
    };
}

from_signed_int!(i8, 1);
from_unsigned_int!(u8, 1);
from_signed_int!(i16, 2);
from_unsigned_int!(u16, 2);
from_signed_int!(i32, 4);
from_unsigned_int!(u32, 4);
from_signed_int!(i64, 8);
from_unsigned_int!(u64, 8);
from_signed_int!(i128, 16);
from_unsigned_int!(u128, 16);
