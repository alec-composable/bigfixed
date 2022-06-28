use crate::{digit::*, Index, IndexError, Cutoff, cutoff::*};

use core::{fmt, ops as coreops, iter::{repeat}, cmp::{max, min}, convert::From, slice::{IterMut}};

pub mod index_ops;
pub mod convert;
pub mod ops;
pub mod ops_c;
pub mod exp;

#[derive(Clone, Copy, Debug)]
pub enum BigFixedError {
    IndexError(IndexError)
}

impl From<IndexError> for BigFixedError {
    fn from(x: IndexError) -> BigFixedError {
        BigFixedError::IndexError(x)
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
            let len = self.body.len();
            while low < len && self.body[low] == 0 {
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
        let shifted_low = (low - self.position)?;
        let shifted_high = (high - self.position)?;
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
                self.body.resize(self.body.len() + add_high, self.head);
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // same as ensure_valid_range where range is position..=position
    pub fn ensure_valid_position(&mut self, position: Index) -> Result<bool, BigFixedError> {
        let p = position.cast_to_position();
        self.ensure_valid_range(p, (p + Index::Position(1))?)
    }

    pub fn range_mut_iter(&mut self, low: Index, high: Index) -> Result<IterMut<Digit>, BigFixedError> {
        self.ensure_valid_range(low, high)?;
        Ok(self.body.iter_mut())
    }

    pub fn range_iter(&self, low: Index, high: Index) -> Result<impl Iterator<Item = Digit> + '_,BigFixedError> {
        assert!(self.properly_positioned());
        let body_high = self.body_high()?;
        let low = low.cast_to_position();
        let keep_low = min(body_high, max(self.position, low));
        let keep_high = min(high, body_high);
        let high = high.cast_to_position();
        Ok(
            repeat(0).take((self.position - low)?.unsigned_value())
            .chain(
                self.body.iter().map(|x| *x)
                .skip((keep_low - self.position)?.unsigned_value())
                .take((keep_high - keep_low)?.unsigned_value())
            )
            .chain(
                repeat(self.head).take((high - body_high)?.unsigned_value())
            )
        )
    }

    pub fn is_neg(&self) -> bool {
        self.head != 0
    }

    // the least position which is outside of the range contained in body
    pub fn body_high(&self) -> Result<Index, BigFixedError> {
        match self.position + self.body.len() {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into())
        }
    }

    pub fn valid_range(&self) -> Result<coreops::Range<Index>, BigFixedError> {
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

    pub fn clear(&mut self) {
        self.overwrite(&BigFixed::ZERO);
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
            (None, Some(floating)) => Ok(max(self.position, (self.greatest_bit_position()? - max(floating, Index::Bit(0)))?)),
            (Some(fixed), Some(floating)) => Ok(min(
                max(self.position, fixed),
                max(self.position, (self.greatest_bit_position()? - max(floating, Index::Bit(0)))?))
            )
        }
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

    pub fn cutoff(&mut self, cutoff: Cutoff) -> Result<(), BigFixedError> {
        self.fix_position()?;
        let cutoff_index = self.cutoff_index(cutoff)?;
        let as_bit = cutoff_index.cast_to_bit()?;
        let as_pos = cutoff_index.cast_to_position();
        let increment = match cutoff.round {
            Rounding::Floor => false,
            Rounding::Round => self[(as_bit - Index::Bit(1))?] > 0,
            Rounding::Ceiling => {
                if self.position >= as_bit {
                    false
                } else {
                    let mut has_value = false;
                    for p in self.position.value()..as_pos.value() {
                        if self[Index::Position(p)] != 0 {
                            has_value = true;
                            break;
                        }
                    }
                    let diff = as_bit.bit_position_excess();
                    if diff > 0 {
                        has_value = has_value || (self[as_pos] & (ALLONES >> (DIGITBITS as isize - diff)) > 0);
                    }
                    has_value
                }
            },
            Rounding::TowardsZero => {
                if self.is_neg() {
                    return self.cutoff(Cutoff {
                        fixed: cutoff.fixed,
                        floating: cutoff.floating,
                        round: Rounding::Floor
                    });
                } else {
                    return self.cutoff(Cutoff {
                        fixed: cutoff.fixed,
                        floating: cutoff.floating,
                        round: Rounding::Ceiling
                    })
                }
            },
            Rounding::AwayFromZero => {
                if self.is_neg() {
                    return self.cutoff(Cutoff {
                        fixed: cutoff.fixed,
                        floating: cutoff.floating,
                        round: Rounding::Ceiling
                    });
                } else {
                    return self.cutoff(Cutoff {
                        fixed: cutoff.fixed,
                        floating: cutoff.floating,
                        round: Rounding::Floor
                    })
                }
            }
        };
        if as_pos > self.position {
            self.body.drain(0..min(self.body.len(), (as_pos - self.position)?.into()));
            self.position = as_pos;
        }
        let diff = (as_bit - as_pos)?.value();
        if diff > 0 {
            if self.body.len() == 0 {
                self.body.push(self.head);
            }
            self[as_pos] &= ALLONES << diff;
        }
        if increment {
            self.add_digit(1, as_bit)?;
        }
        self.format()?;
        Ok(())
    }

    pub const ZERO: BigFixed = BigFixed {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    };
}

impl CutsOff for BigFixed {
    fn cutoff(&mut self, cutoff: Cutoff) -> Result<(), BigFixedError> {
        self.cutoff(cutoff)
    }
}

impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_neg() {
            return write!(f, "-{}", (-self).unwrap());
        };
        let (mut digits, mut point) = self.to_digits_10().unwrap();
        point = Index::castsize(digits.len()).unwrap() - point;
        digits.reverse();
        if point == 0 {
            write!(f, "0").ok();
        } else if point < 0 {
            write!(f, "0.{}", "0".repeat(Index::uncastsize(-point).unwrap())).ok();
        }
        for d in digits {
            if point == 0isize {
                write!(f, ".").ok();
            }
            point -= 1isize;
            write!(f, "{}", d).ok();
        }
        write!(f, "{}", "0".repeat(Index::saturating_unsigned(point))).ok();
        write!(f, "")
    }
}

impl fmt::Binary for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_neg() {
            return write!(f, "-{:b}", (-self).unwrap());
        };
        let start = self.greatest_bit_position().unwrap().bit_value().unwrap();
        if start < 0 {
            write!(f, "{}", "0".repeat(Index::uncastsize(-start).unwrap())).ok();
            write!(f, ".").ok();
        };
        for p in (self.position.bit_value().unwrap()..=start).rev() {
            //println!("p {}", p);
            write!(f, "{}", self[Index::Bit(p)]).ok();
            if p == 0 {
                write!(f, ".").ok();
            }
        }
        Ok(())
    }
}
