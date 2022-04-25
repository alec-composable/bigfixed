use crate::{digit::*, Index, Cutoff, CutsOff, BigFixed};

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

    // TODO: float special cases -- zeroess, infinities, NANs

    // load float into a BigFixed as an unsigned integer with identical bits then call this to interpret it correctly
    // float format: [0][sign bit][exponent + bias][significand].[0]
    pub fn float_from_bits(mut self, exponent_len: usize, exponent_bias: isize, significand_len: usize) -> BigFixed {
        assert!(!self.is_neg() && self.position >= 0isize, "improper float format");
        // get sign
        self >>= exponent_len + significand_len;
        let is_neg = self[0] == 1;
        self[0] = 0;
        // get exponent
        self <<= exponent_len;
        let exp = <usize>::from(&self) as isize - exponent_bias;
        for i in Index::ZERO.to(&self.body_high()) {
            self[i] = 0;
        }
        // introduct implicit significand greatest bit (except for the special case 0)
        if self.is_zero() && exp == exponent_bias {
            self.format();
            return self;
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
    }

    // recasts self as an unsigned integer whose bits match the specified pattern, saturating the exponent and truncating the significand
    pub fn float_to_bits(mut self, exponent_len: usize, exponent_bias: isize, significand_len: usize) -> BigFixed {
        if self.is_zero() {
            return self;
        }
        let neg = self.is_neg();
        if neg {
            self.negate();
        };
        // head is 0
        let position = self.greatest_bit_position();
        if position >= 0 {
            self >>= position as usize;
        } else {
            self <<= (-position) as usize;
        }
        // self.int() == 1, pop it because it is implied in float format
        self.body.pop();
        let low_pos = -Index::from((significand_len + DIGITBITS - 1) / DIGITBITS);
        self.cutoff(Cutoff{
            // ceiling division
            fixed: Some(low_pos),
            floating: None
        });
        if self.body.len() > 0 {
            self[low_pos] &= ALLONES << (DIGITBITS - (significand_len % DIGITBITS));
        }
        let mut exp = isize::from(position + exponent_bias);
        let max_exp: isize = usize::from(ALLONES >> (DIGITBITS - exponent_len)).try_into().unwrap();
        // saturation cases
        if exp < 0 {
            // exponent too low, setting to zero
            exp = 0;
            self.cutoff(Cutoff{
                fixed: Some(Index::ZERO),
                floating: None
            });
        } else if exp > max_exp {
            // exponent too high, setting to infty
            exp = max_exp;
            self.cutoff(Cutoff{
                fixed: Some(Index::ZERO),
                floating: None
            });
        }
        self += BigFixed::from(exp);
        self >>= exponent_len;
        if neg {
            self += BigFixed::from(1);
        }
        self <<= exponent_len + significand_len;
        self
    }
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
                    // saturating: too high
                    (cutoff - 1) as $int
                } else {
                    c.negate();
                    if x < &c {
                        // saturating: too low
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

macro_rules! from_float {
    ($type: ty, $exponent_len: expr, $exponent_bias: expr, $significand_len: expr) => {
        impl From<$type> for BigFixed {
            fn from(x: $type) -> BigFixed {
                let mut returner = BigFixed::from(x.to_bits());
                returner = returner.float_from_bits($exponent_len, $exponent_bias, $significand_len);
                returner
            }
        }
    };
}

from_float!(f32, 8, 127, 23);
from_float!(f64, 11, 1023, 52);

macro_rules! to_float {
    ($type: ty, $unsigned_type: ty, $exponent_len: expr, $exponent_bias: expr, $significand_len: expr) => {
        impl From<BigFixed> for $type {
            fn from(mut x: BigFixed) -> $type {
                x = x.float_to_bits($exponent_len, $exponent_bias, $significand_len);
                <$type>::from_bits(
                    <$unsigned_type>::from(&x)
                )
            }
        }
    }
}

to_float!(f32, u32, 8, 127, 23);
to_float!(f64, u64, 11, 1023, 52);
