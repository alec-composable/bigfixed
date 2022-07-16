use crate::{
    Digit,
    Index,
    Cutoff,
    CutsOff,
    BigFixed,
    BigFixedError
};

use std::{convert::{From}, cmp::{max}};

impl<D: Digit> From<D> for BigFixedVec<D> {
    fn from(x: D) -> BigFixedVec<D> {
        BigFixedVec {
            head: D::ZERO,
            body: vec![x],
            position: Index::Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        }
    }
}

impl<D: Digit> BigFixedVec<D> {
    // little endian bytes
    pub fn int_from_bytes(bytes: &[u8], unsigned: bool) -> Result<BigFixedVec<D>, BigFixedError> {
        // extension stuff in case bytes length does not divide DIGITBYTES evenly
        let good_len = bytes.len() / D::DIGITBYTES;
        let good_bytes_len = good_len * D::DIGITBYTES;
        // one extra for byte-extended digit
        let mut data: Vec<D> = Vec::with_capacity(good_len + 1);
        data.extend(
            (0..good_len).map(
                |i| i * D::DIGITBYTES
            ).map(
                |j| D::digit_from_bytes(
                    &bytes[j..(j + D::DIGITBYTES)]
                )
            )
        );
        if bytes.len() != good_bytes_len {
            // have to extend the greatest digit's bytes
            let fill: u8 = if !unsigned && bytes[bytes.len()-1] >= 128 {255} else {0};
            let mut int_bytes: Vec<u8> = Vec::with_capacity(D::DIGITBYTES);//[fill; D::DIGITBYTES];
            int_bytes.resize(D::DIGITBYTES, fill);
            let fill_len = bytes.len() - good_bytes_len;
            int_bytes[0..fill_len].clone_from_slice(&bytes[good_bytes_len..bytes.len()]);
            data.push(D::digit_from_bytes(&int_bytes));
        }
        let is_neg = !unsigned && data.len() > 0 && data[data.len()-1] >= D::GREATESTBIT;
        BigFixed::construct(
            if is_neg {D::ALLONES} else {D::ZERO},
            data,
            Index::Position(0)
        )
    }

    // TODO: float special cases -- zeroes, infinities, NANs

    // load float into a BigFixed as an unsigned integer with identical bits then call this to interpret it correctly
    // float format: [0][sign bit][exponent + bias][significand].[0]
    pub fn float_from_bits(mut self, exponent_len: usize, exponent_bias: isize, significand_len: usize) -> Result<BigFixedVec<D>, BigFixedError> {
        assert!(!self.is_neg()? && self.position >= 0isize, "improper float format");
        // get sign
        self = self.shift(Index::Bit(-Index::<D>::castsize(exponent_len + significand_len)?))?;
        let is_neg = self[0] == D::ONE;
        self[0] = D::ZERO;
        // get exponent
        self = self.shift(Index::Bit(Index::<D>::castsize(exponent_len)?))?;
        let exp = Index::<D>::castsize(<usize>::from(&self))? - exponent_bias;
        for i in 0..max(0, self.body_high()?.value()?) {
            self[i] = D::ZERO;
        }
        // introduct implicit significand greatest bit (except for the special case 0)
        if self.is_zero()? && exp == exponent_bias {
            self.format()?;
            return Ok(self);
        }
        self[0] = D::ONE;
        self = self.shift(Index::Bit(exp))?;
        if is_neg {
            self.negate()?;
        }
        Ok(self)
    }

    // recasts self as an unsigned integer whose bits match the specified pattern, saturating the exponent and truncating the significand
    pub fn float_to_bits(mut self, exponent_len: usize, exponent_bias: isize, significand_len: usize) -> Result<BigFixedVec<D>, BigFixedError> {
        if self.is_zero()? {
            return Ok(self);
        }
        let neg = self.is_neg()?;
        if neg {
            self.negate()?;
        };
        // head is 0
        let position = self.greatest_bit_position()?;
        self = self.shift((-position)?)?;
        // self.int() == 1, pop it because it is implied in float format
        self.body.pop();
        let low_pos = (-Index::<D>::Bit(Index::<D>::castsize(significand_len + D::DIGITBITS - 1)?))?;
        self.cutoff(Cutoff{
            fixed: Some(low_pos),
            floating: None,
            round: Rounding::Floor
        })?;
        if self.body.len() > 0 {
            self[low_pos] &= D::ALLONES << (D::DIGITBITS - (significand_len % D::DIGITBITS));
        }
        let mut exp = (position + exponent_bias)?.value()?;
        let max_exp = Index::<D>::castsize((D::ALLONES >> (D::DIGITBITS - exponent_len)).into())?;
        // saturation cases
        if exp < 0 {
            // exponent too low, setting to zero
            exp = 0;
            self.clear();
        } else if exp > max_exp {
            // exponent too high, setting to infty
            exp = max_exp;
            self.clear();
        }
        self |= &BigFixedVec::from(exp);
        self = self.shift((-Index::Bit(Index::<D>::castsize(exponent_len)?))?)?;
        if neg {
            self |= &BigFixedVec::<D>::from(D::ONE);
        }
        self = self.shift((Index::Bit(Index::<D>::castsize(exponent_len)?) + Index::Bit(Index::<D>::castsize(significand_len)?))?)?;
        Ok(self)
    }
}

impl<D: Digit> From<&[u8]> for BigFixedVec<D> {
    fn from(bytes: &[u8]) -> BigFixedVec<D> {
        BigFixedVec::int_from_bytes(bytes, true).unwrap()
    }
}

impl<D: Digit> From<&[i8]> for BigFixedVec<D> {
    fn from(bytes: &[i8]) -> BigFixedVec<D> {
        let bytes: Vec<u8> = bytes.iter().map(|b| *b as u8).collect();
        BigFixedVec::int_from_bytes(bytes.as_slice(), false).unwrap()
    }
}

macro_rules! from_signed_int {
    ($s: ty, $n: expr) => {
        impl<D: Digit> From<$s> for BigFixedVec<D> {
            fn from(i: $s) -> BigFixedVec<D> {
                    BigFixedVec::int_from_bytes(&i.to_le_bytes() as &[u8], false).unwrap()
            }
        }
    };
}

macro_rules! from_unsigned_int {
    ($u: ty, $num_bytes: expr) => {
        impl<D: Digit> From<$u> for BigFixedVec<D> {
            fn from(u: $u) -> BigFixedVec<D> {
                BigFixedVec::int_from_bytes(&u.to_le_bytes() as &[u8], true).unwrap()
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
        impl<D: Digit> From<&BigFixedVec<D>> for $int {
            fn from(x: &BigFixedVec<D>) -> $int {
                if D::DIGITBYTES >= $num_bytes {
                    let mut bytes = [0u8; $num_bytes];
                    let d = x[0].to_le_bytes();
                    bytes[0..$num_bytes].copy_from_slice(&d[0..$num_bytes]);
                    <$int>::from_le_bytes(bytes)
                } else {
                    assert_eq!($num_bytes % D::DIGITBYTES, 0, "byte number mismatch");
                    let len = $num_bytes / D::DIGITBYTES;
                    let mut bytes = [0u8; $num_bytes];
                    let mut on = 0;
                    for i in 0..len {
                        let d = x[i as isize].to_le_bytes();
                        for j in 0..D::DIGITBYTES {
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
        impl<D: Digit> From<&BigFixedVec<D>> for $int {
            fn from(x: &BigFixedVec<D>) -> $int {
                let cutoff: $unsigned = 1 as $unsigned << (8 * $num_bytes - 1);
                let mut c = BigFixedVec::<D>::from(cutoff);
                if x >= &c {
                    // saturating: too high
                    (cutoff - 1) as $int
                } else {
                    c.negate().unwrap();
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
        impl<D: Digit> From<$type> for BigFixedVec<D> {
            fn from(x: $type) -> BigFixedVec<D> {
                let mut returner = BigFixedVec::<D>::from(x.to_bits());
                returner = returner.float_from_bits($exponent_len, $exponent_bias, $significand_len).unwrap();
                returner
            }
        }
    };
}

from_float!(f32, 8, 127, 23);
from_float!(f64, 11, 1023, 52);

macro_rules! to_float {
    ($type: ty, $unsigned_type: ty, $exponent_len: expr, $exponent_bias: expr, $significand_len: expr) => {
        impl<D: Digit> From<BigFixedVec<D>> for $type {
            fn from(mut x: BigFixedVec<D>) -> $type {
                x = x.float_to_bits($exponent_len, $exponent_bias, $significand_len).unwrap();
                <$type>::from_bits(
                    <$unsigned_type>::from(&x)
                )
            }
        }

        impl<D: Digit> From<&BigFixedVec<D>> for $type {
            fn from(x: &BigFixedVec<D>) -> $type {
                <$type>::from(x.clone())
            }
        }
    }
}

to_float!(f32, u32, 8, 127, 23);
to_float!(f64, u64, 11, 1023, 52);
