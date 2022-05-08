use crate::{digit::*, Index, IndexError, Cutoff, CutsOff};

use std::{fmt, ops as stdops, iter::{repeat}, cmp::{max, min}, convert::From};

pub mod index_ops;
pub mod convert;
pub mod ops;
//pub mod ops_c;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BigFixedError {
    IndexError(IndexError)
}

pub use BigFixedError::{
    IndexError as BigFixedIndexError
};

impl From<IndexError> for BigFixedError {
    fn from(x: IndexError) -> BigFixedError {
        BigFixedIndexError(x)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigFixed {
    pub head: Digit,
    pub body: Vec<Digit>,
    pub position: Index
}

impl BigFixed {
    // fix position then remove redundant body data
    pub fn format(&mut self) -> Result<(), BigFixedError> {
        if self.head != 0 {
            self.head = ALLONES;
        }
        self.fix_position()?;
        let mut high = self.body.len();
        while high > 0 && self.body[high - 1] == self.head {
            high -= 1;
        }
        self.body.truncate(high);
        if self.body.len() > 0 {
            let mut low = 0;
            while self.body[low] == 0 {
                low += 1;
            }
            self.body.drain(0..low);
            self.position += low;
        }
        // special case: zero
        if self.head == 0 && self.body.len() == 0 {
            self.position = Index::Position(0);
        }
        Ok(())
    }

    // If self.position is Index::Bit this will bit shift as necessary and turn it into Index::Position. Returns whether casting was necessary.
    pub fn fix_position(&mut self) -> Result<bool, BigFixedError> {
        let changed;
        match self.position {
            Index::Position(_) => {
                changed = false;
            },
            Index::Bit(b) => {
                let diff = Index::uncastsize(b.rem_euclid(Index::castsize(DIGITBITS)?))?; 
                let p = (b - Index::castsize(diff)?) / Index::castsize(DIGITBITS)?;
                // b = p*DIGITBITS + diff and 0 <= diff < DIGITBITS
                // shift left diff and change position to p
                if diff > 0 {
                    let opdiff = DIGITBITS - diff;
                    let keepmask: Digit = ALLONES >> diff;
                    let carrymask: Digit = !keepmask;
                    let len = self.body.len();
                    if len > 0 {
                        let high_digit = ((self.head & keepmask) << diff) | ((self.body[len - 1] & carrymask) >> opdiff);
                        if high_digit != self.head {
                            self.body.push(high_digit);
                        }
                        for i in (1..len).rev() {
                            self.body[i] = ((self.body[i] & keepmask) << diff) | ((self.body[i-1] & carrymask) >> opdiff);
                        }
                        self.body[0] = (self.body[0] & keepmask) << diff;
                    } else if self.is_neg() {
                        self.body.push((ALLONES & keepmask) << diff);
                    }
                }
                self.position = Index::Position(p);
                changed = true;
            }
        }
        Ok(changed)
    }

    pub fn properly_positioned(&self) -> bool {
        match self.position {
            Index::Position(_) => true,
            Index::Bit(_) => false
        }
    }

    pub fn format_c(&mut self, cutoff: Cutoff) -> Result<(), BigFixedError> {
        self.format()?;
        if self.body.len() == 0 && !self.is_zero() {
            match cutoff.fixed {
                None => {},
                Some(x) => if self.position < x {
                    self.position = x;
                }
            }
        }
        Ok(())
    }

    pub fn construct(head: Digit, body: Vec<Digit>, position: Index) -> Result<BigFixed, BigFixedError> {
        let mut returner = BigFixed {
            head,
            body,
            position
        };
        returner.format()?;
        Ok(returner)
    }

    // Restructure if necessary so that all positions in low..high are valid. Breaks format so reformat afterwards. Returns whether restructuring was necessary.
    pub fn ensure_valid_range(&mut self, low: Index, high: Index) -> Result<bool, BigFixedError> {
        self.fix_position()?;
        if low >= high {
            if low == high {
                return Ok(false);
            } else {
                return self.ensure_valid_range(high, low);
            }
        }
        let low = low.cast_to_position();
        let high = high.cast_to_position();
        let shifted_low = (low.cast_to_position() - self.position)?;
        let shifted_high = (high.cast_to_position() - self.position)?;
        let add_low = (-shifted_low)?.unsigned_value();
        let add_high = (shifted_high - Index::castsize(self.body.len())?)?.unsigned_value();
        self.position = min(low, self.position);
        let reserve = add_low + add_high;
        if reserve > 0 {
            self.body.reserve(reserve);
            if add_low > 0 {
                self.body.splice(0..0, repeat(0).take(add_low));
            }
            if add_high > 0 {
                self.body.resize(shifted_high.into(), self.head);
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // same as ensure_valid_range where range is position..=position
    pub fn ensure_valid_position(&mut self, position: Index) -> Result<bool, BigFixedError> {
        let p = position.cast_to_position();
        self.ensure_valid_range(p, (p + 1isize)?)
    }

    pub fn is_neg(&self) -> bool {
        self.head != 0
    }

    // the least position which is outside of the range contained in body
    pub fn body_high(&self) -> Result<Index, BigFixedError> {
        match self.position + self.body.len() {
            Ok(res) => Ok(res),
            Err(e) => Err(BigFixedIndexError(e))
        }
    }

    pub fn valid_range(&self) -> Result<stdops::Range<Index>, BigFixedError> {
        Ok(self.position..self.body_high()?)
    }

    pub fn int(&self) -> Result<BigFixed, BigFixedError> {
        BigFixed::construct(
            self.head,
            self.body[(-self.position)?.unsigned_value()..self.body.len()].to_vec(),
            self.position.saturating_nonnegative()
        )
    }

    pub fn frac(&self) -> Result<BigFixed, BigFixedError> {
        BigFixed::construct(
            0,
            self.body[0..(-self.position)?.unsigned_value()].to_vec(),
            self.position // if position is positive then body must be empty and format() resets position to 0
        )
    }

    pub fn overwrite(&mut self, src: &BigFixed) {
        self.head = src.head;
        self.body.splice(0..self.body.len(), src.body.iter().map(|x| *x));
        self.position = src.position;
    }

    pub fn shift(mut self, shift: Index) -> Result<BigFixed, BigFixedError> {
        self.position += shift;
        self.format()?;
        Ok(self)
    }

    // does not require proper formatting -- fully checks if self is zero
    pub fn is_zero(&self) -> bool {
        self.head == 0 && self.body.iter().all(|&x| x == 0)
    }

    pub fn full_eq(&self, other: &BigFixed) -> Result<bool, BigFixedError> {
        for i in min(
            self.position.cast_to_position(),
            other.position.cast_to_position()
        ).value()..(
            max(
                self.body_high()?,
                other.body_high()?
            ) + 1isize
        )?.value() {
            if self[Index::Position(i)] != other[Index::Position(i)] {
                return Ok(false);
            }
        };
        Ok(true)
    }

    pub fn cutoff_index(&self, cutoff: Cutoff) -> Result<Index, BigFixedError> {
        match (cutoff.fixed, cutoff.floating) {
            (None, None) => Ok(self.position), // no cutoff
            (Some(fixed), None) => Ok(max(self.position, fixed)),
            (None, Some(floating)) => Ok(max(
                self.position,
                ((self.greatest_bit_position()? + Index::Bit(1))? - max(floating, Index::Bit(0)))?
            )),
            (Some(fixed), Some(floating)) => Ok(min(
                max(self.position, fixed),
                max(
                    self.position,
                    ((self.greatest_bit_position()? + Index::Bit(1))? - max(floating, Index::Bit(0)))?
                ))
            )
        }
    }

    pub fn cutoff_fixed_bit(&mut self, b: isize) -> Result<(), BigFixedError> {
        self.cutoff(
            Cutoff {
                fixed: Some(Index::Bit(b)),
                floating: None
            }
        )
    }

    pub fn cutoff_fixed_position(&mut self, p: isize) -> Result<(), BigFixedError> {
        self.cutoff(
            Cutoff {
                fixed: Some(Index::Position(p)),
                floating: None
            }
        )
    }

    pub fn cutoff_floating_bit(&mut self, b: isize) -> Result<(), BigFixedError> {
        self.cutoff(
            Cutoff {
                fixed: None,
                floating: Some(Index::Bit(b))
            }
        )
    }

    pub fn cutoff_floating_position(&mut self, p: isize) -> Result<(), BigFixedError> {
        self.cutoff(
            Cutoff {
                fixed: None,
                floating: Some(Index::Position(p))
            }
        )
    }

    pub fn greatest_bit_position(&self) -> Result<Index, BigFixedError> {
        // zero is special, just return 0
        if self.is_zero() {
            return Ok(Index::Position(0));
        }
        let position = self.body_high()?;
        let coefficient: Digit = self[(position - 1isize)?] ^ self.head; // greatest bit which differs from head is greatest bit here
        Ok(Index::Bit(position.bit_value()? - Index::castsize(coefficient.leading_zeros() as usize + 1)?))
    }

    pub const ZERO: BigFixed = BigFixed {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    };
}

impl CutsOff for BigFixed {
    fn cutoff(&mut self, cutoff: Cutoff) -> Result<(), BigFixedError> {
        //println!("{} cutting off {}", self, cutoff);
        self.fix_position()?;
        //println!("fixed pos {}", self);
        let cutoff_index = self.cutoff_index(cutoff)?;
        //println!("cutoff index {}", cutoff_index);
        let as_bit = cutoff_index.cast_to_bit()?;
        let as_pos = cutoff_index.cast_to_position();
        //println!("that is {} {}", as_bit, as_pos);
        self.position = min(self.position, as_pos);
        //println!("cut off tail {}", self);
        if as_pos > self.position {
            //println!("draining {}", min(self.body.len(), (as_pos - self.position)?.into()));
            self.body.drain(0..min(self.body.len(), (as_pos - self.position)?.into()));
            self.position = as_pos;
            //println!("drained body {}", self);
        }
        let diff = (as_bit - as_pos)?.value();
        if diff > 0 {
            //println!("diffing {}", diff);
            if self.body.len() == 0 {
                self.body.push(self.head);
            }
            let len = self.body.len();
            self.body[len - 1] &= ALLONES << diff;
        }
        self.format()?;
        Ok(())
    }
}

impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*let (mut digits, mut point) = self.to_digits_10();
        point = (digits.len() - point).unwrap();
        digits.reverse();
        if point == 0isize {
            write!(f, "0").ok();
        }
        for d in digits {
            if point == 0isize {
                write!(f, ".").ok();
            }
            point -= 1isize;
            write!(f, "{}", d).ok();
        }
        write!(f, "")*/
        write!(f, "{:?}", self)
    }
}

impl fmt::Binary for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut body_rev = self.body.clone();
        body_rev.reverse();
        write!(f, "{:b} [", self.head).ok();
        let mut first = true;
        for x in body_rev {
            if first {
                first = false;
            } else {
                write!(f, ", ").ok();
            }
            write!(f, binary_formatter!(), x).ok();
        }
        write!(f, "] position {}", self.position)
    }
}
