use crate::{digit::*, Index, BigFixed};

use std::{convert::{From}};

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
                |j| digit_from_bytes(
                    &bytes[j..(j+DIGITBYTES)]
                )
            )
        );
        if bytes.len() != good_bytes_len {
            // have to extend the greatest digit's bytes
            let fill: u8 = if !unsigned && bytes[bytes.len()-1] >= 128 {255} else {0};
            let mut int_bytes = [fill; DIGITBYTES];
            let fill_len = bytes.len() - good_bytes_len;
            int_bytes[0..fill_len].clone_from_slice(&bytes[good_bytes_len..bytes.len()]);
            data.push(digit_from_bytes(&int_bytes));
        }
        let is_neg = !unsigned && data.len() > 0 && data[data.len()-1] >= GREATESTBIT;
        BigFixed::construct(
            if is_neg {ALLONES} else {0},
            data,
            Index::ZERO
        )
    }

    // load float into a BigFixed as an unsigned integer with identical bits then call this to interpret it correctly
    // float format: [0][sign bit][exponent + bias][significand].[0]
    /*pub fn float_from_bits(mut self, exponent_len: usize, exponent_bias: isize, significand_len: usize) -> BigFixed {
        assert!(!self.is_neg() && self.position >= 0isize, "improper float format");
        self >>= exponent_len + significand_len;
        let is_neg = self[0] == 1;
        self[0] = 0;
        self <<= exponent_len;
        let exp = <usize>::from(&self) as isize - exponent_bias;
        for i in Index::ZERO.to(&self.body_high()) {
            self[i] = 0;
        }
        self[0] = 1;
        if exp < 0 {
            self >>= (-exp) as usize;
        } else {
            self <<= exp as usize;
        }
        if is_neg {
            self.negate();
        }
        self
    }*/
}

macro_rules! from_signed_int {
    ($s: ty, $n: expr) => {
        impl From<$s> for BigFixed {
            fn from(i: $s) -> BigFixed {
                    BigFixed::int_from_bytes(&i.to_le_bytes() as &[u8], false)
            }
        }
    };
}

macro_rules! from_unsigned_int {
    ($u: ty, $num_bytes: expr) => {
        impl From<$u> for BigFixed {
            fn from(u: $u) -> BigFixed {
                BigFixed::int_from_bytes(&u.to_le_bytes() as &[u8], true)
            }
        }
    };
}

const SIZEBYTES: usize = (usize::BITS / 8) as usize;

from_signed_int!(isize, SIZEBYTES);
from_unsigned_int!(usize, SIZEBYTES);
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

// to_unsigned_int is a bit casting over the bits of BigFixed::from(ALLONES)

macro_rules! to_unsigned_int {
    ($int: ty, $num_bytes: expr) => {
        impl From<&BigFixed> for $int {
            fn from(x: &BigFixed) -> $int {
                if DIGITBYTES >= $num_bytes {
                    let mut bytes = [0u8; $num_bytes];
                    let d = x[0].to_le_bytes();
                    bytes[0..$num_bytes].copy_from_slice(&d[0..$num_bytes]);
                    <$int>::from_le_bytes(bytes)
                } else {
                    assert_eq!($num_bytes % DIGITBYTES, 0, "byte number mismatch");
                    let len = $num_bytes / DIGITBYTES;
                    let mut bytes = [0u8; $num_bytes];
                    let mut on = 0;
                    for i in 0..len {
                        let d = x[i as isize].to_le_bytes();
                        for j in 0..DIGITBYTES {
                            bytes[on] = d[j];
                            on += 1;
                        }
                    }
                    <$int>::from_le_bytes(bytes)
                }
            }
        }
    };
}

to_unsigned_int!(usize, SIZEBYTES);
to_unsigned_int!(u8, 1);
to_unsigned_int!(u16, 2);
to_unsigned_int!(u32, 4);
to_unsigned_int!(u64, 8);
to_unsigned_int!(u128, 16);

// to_signed_int is a saturating cast

macro_rules! to_signed_int {
    ($int: ty, $unsigned: ty, $num_bytes: expr) => {
        impl From<&BigFixed> for $int {
            fn from(x: &BigFixed) -> $int {
                let cutoff: $unsigned = 1 as $unsigned << (8 * $num_bytes - 1);
                let mut c = BigFixed::from(cutoff);
                if x >= &c {
                    (cutoff - 1) as $int
                } else {
                    c.negate();
                    if x < &c {
                        cutoff as $int
                    } else {
                        <$unsigned>::from(x) as $int
                    }
                }
            }
        }
    };
}

to_signed_int!(isize, usize, SIZEBYTES);
to_signed_int!(i8, u8, 1);
to_signed_int!(i16, u16, 2);
to_signed_int!(i32, u32, 4);
to_signed_int!(i64, u64, 8);
to_signed_int!(i128, u128, 16);

/*macro_rules! from_float {
    ($type: ty, $exponent_len: expr, $exponent_bias: expr, $significand_len: expr) => {
        impl From<$type> for BigFixed {
            fn from(x: $type) -> BigFixed {
                BigFixed::from(x.to_bits()).float_from_bits($exponent_len, $exponent_bias, $significand_len)
            }
        }
    };
}

from_float!(f32, 8, 127, 23);
from_float!(f64, 11, 1023, 52);*/
