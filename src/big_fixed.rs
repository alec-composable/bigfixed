use crate::digit::*;

use core::{
    fmt
};

use std::error::Error;

//pub mod big_fixed_vec;
//pub use big_fixed_vec::BigFixedVec;
//pub mod position;
//pub mod range;
//pub mod cutoff;
//pub mod ops {
//    pub mod index;
//    pub mod addition;
//}
//pub mod convert {
//    pub mod int;
//}
//pub mod ops;
//pub mod ops_c;
//pub mod exp;


#[derive(Clone, Copy, Debug)]
pub enum BigFixedError {
    OverflowError
}

impl fmt::Display for BigFixedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BigFixedError {}

#[derive(Clone, Copy, Debug)]
pub struct BigFixed<D: Digit> {
    pub is_positive: bool,
    pub lesser: D,
    pub greater: D,
    pub position: Index
}

impl<D: Digit> BigFixed<D> {
    pub fn greater_trivial_bits(self) -> Index {
        if self.is_positive {
            self.greater.leading_zeros()
        } else {
            self.greater.leading_ones()
        }
    }

    pub fn lesser_trivial_bits(self) -> Index {
        self.lesser.trailing_zeros()
    }

    pub fn greatest_bit_position(&self) -> Index {
        self.position + (D::DIGITBITSI - self.greater_trivial_bits())
    }

    pub fn least_bit_position(&self) -> Index {
        self.position + self.lesser.trailing_zeros()
    }
}

pub trait BigDigit: Digit {

}

pub trait BigGrowableDigit: GrowableDigit + BigDigit {

}

pub trait BigShrinkableDigit: ShrinkableDigit + BigDigit {

}

pub trait BigMalleableDigit: BigGrowableDigit + BigShrinkableDigit {}