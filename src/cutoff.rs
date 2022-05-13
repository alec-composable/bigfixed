/*
    A cutoff is a pair of optional Indexes (n, m) where m >= 0. n is the fixed cutoff and m is the floating cutoff.

    A cutoff with (None, None) is no restriction. Cutting off wrt (None, None) leaves the number unchanged.
    
    A cutoff with (n, None) where n is an Index corresponds to fixed point arithmetic with positions >= n.
    For a number x with positional representation (b_k) (k ranges over Index) the fixed cutoff is a guarantee that no b_k for k >= n ever gets cut off.
    n is a lower bound for lossless addition -- if x and y are two BigFixeds with positions >= n then x + y is lossless.

    A cutoff with (None, m) corresponds to floating point arithmetic with significand width m. In this scheme the head corresponds to the sign bit and
    the implied most significant bit. The floating cutoff is a lower bound on word size. In numbers like 0.00000232321423... which are below the fixed
    cutoff and which do not truncate quickly (or ever), the value of m states how many nontrivial coefficients to keep.

    Together the BigFixed cutoff scheme with respect to (n, m) is like floating point behavior with significand width m combined with BigInt fixed point
    behavior for positions at and above n. This ensures lossless additive structure above the fixed cutoff while maintaining floating multiplicative
    integrity consistent with the floating cutoff for very small values. Large values are unaffected by the cutoff and care must be taken to ensure that
    they truncate quickly enough lest they devour available resources.
*/

use crate::{Index, BigFixedError};

use std::{cmp::{PartialEq}, fmt};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rounding {
    Floor,
    Ceiling,
    Round,
    TowardsZero,
    AwayFromZero
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Cutoff {
    pub fixed: Option<Index>,
    pub floating: Option<Index>,
    pub round: Rounding
}

pub trait CutsOff {
    fn cutoff(&mut self, cutoff: Cutoff) -> Result<(), BigFixedError>;
}

impl fmt::Display for Cutoff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.fixed, self.floating)
    }
}
