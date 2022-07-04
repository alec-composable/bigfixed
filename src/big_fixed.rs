use crate::{digit::*, Index, IndexError, cutoff::*};

use core::{fmt, /*ops as coreops,*/ ops::{Range}, iter::{repeat/*, Repeat, Chain, Take, Skip, Map*/}, cmp::{max, min}, convert::From, slice::{/*Iter,*/ IterMut}};

use std::error::Error;

use num_traits::PrimInt;

pub mod index_ops;
pub mod convert;
pub mod ops;
//pub mod ops_c;
//pub mod exp;

#[derive(Clone, Copy, Debug)]
pub enum BigFixedError {
    // absorb IndexErrors
    IndexError(IndexError),
    // not applicable to Vec-based BigFixed but applicable to hard coded versions
    OutOfBoundsError
}

impl From<IndexError> for BigFixedError {
    fn from(x: IndexError) -> BigFixedError {
        BigFixedError::IndexError(x)
    }
}

impl fmt::Display for BigFixedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BigFixedError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigFixedVec<D: Digit> {
    pub head: D,
    pub body: Vec<D>,
    pub position: Index<D>,
    // These are just here for ownership rules re ops::Index. They were not necessary when Digit was directly a u* but now they are apparently necessary.
    pub zero_copy: D,
    pub one_copy: D
}

pub trait BigFixed<D: Digit>:
    Clone + PartialEq + Eq
    + CutsOff<D, BigFixedError>
{
    fn head(&self) -> D;

    fn position(&self) -> Index<D>;

    // fix position then remove redundant body data
    fn format(&mut self) -> Result<(), BigFixedError>;

    // If self.position is Index::Bit this will bit shift as necessary and turn it into Index::Position. Returns whether casting was necessary.
    fn fix_position(&mut self) -> Result<bool, BigFixedError>;

    fn construct(head: D, body: Vec<D>, position: Index<D>) -> Result<Self, BigFixedError>;

    // Check if self.position is of type Index::Position
    fn properly_positioned(&self) -> bool {
        match self.position() {
            Index::Position(_) => true,
            Index::Bit(_) => false,
            Index::DigitTypeInUse(_) => false
        }
    }

    // Restructure if necessary so that all positions in low..high are valid. Breaks format so reformat afterwards. Returns whether restructuring was necessary.
    fn ensure_valid_range(&mut self, low: Index<D>, high: Index<D>) -> Result<bool, BigFixedError>;

    // same as ensure_valid_range where range is position..=position
    fn ensure_valid_position(&mut self, position: Index<D>) -> Result<bool, BigFixedError> {
        let p = position.cast_to_position()?;
        self.ensure_valid_range(p, (p + Index::Position(1))?)
    }

    // ensure valid range then return a mutable iterator over the body
    fn range_mut_iter(&mut self, low: Index<D>, high: Index<D>) -> Result<IterMut<D>, BigFixedError>;

    fn is_neg(&self) -> Result<bool, BigFixedError>;

    // the least position which is outside of the range contained in body
    fn body_high(&self) -> Result<Index<D>, BigFixedError>;

    fn valid_range(&self) -> Result<Range<Index<D>>, BigFixedError>;

    fn int(&self) -> Result<Self, BigFixedError>;

    fn frac(&self) -> Result<Self, BigFixedError>;

    fn overwrite(&mut self, src: &Self);

    fn clear(&mut self) {
        self.overwrite(&Self::ZERO);
    }

    fn shift(self, shift: Index<D>) -> Result<Self, BigFixedError>;

    // does not require proper formatting -- fully checks if self is zero
    fn is_zero(&self) -> Result<bool, BigFixedError>;

    fn full_eq(&self, other: &Self) -> Result<bool, BigFixedError>;

    fn cutoff_index(&self, cutoff: Cutoff<D>) -> Result<Index<D>, BigFixedError> {
        match (cutoff.fixed, cutoff.floating) {
            (None, None) => Ok(self.position()), // no cutoff
            (Some(fixed), None) => Ok(max(self.position(), fixed)),
            (None, Some(floating)) => Ok(max(self.position(), (self.greatest_bit_position()? - max(floating, Index::Bit(0)))?)),
            (Some(fixed), Some(floating)) => Ok(min(
                max(self.position(), fixed),
                max(self.position(), (self.greatest_bit_position()? - max(floating, Index::Bit(0)))?))
            )
        }
    }

    fn greatest_bit_position(&self) -> Result<Index<D>, BigFixedError>;

    const ZERO: Self;
}

impl<D: Digit> BigFixed<D> for BigFixedVec<D> {
    fn head(&self) -> D {
        self.head
    }

    fn position(&self) -> Index<D> {
        self.position
    }
    // fix position then remove redundant body data
    fn format(&mut self) -> Result<(), BigFixedError> {
        if self.head != D::ZERO {
            self.head = D::ALLONES;
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
            while low < len && self.body[low] == D::ZERO {
                low += 1;
            }
            self.body.drain(0..low);
            self.position += low;
        }
        // special case: zero
        if self.head == D::ZERO && self.body.len() == 0 {
            self.position = Index::Position(0);
        }
        Ok(())
    }

    // If self.position is Index::Bit this will bit shift as necessary and turn it into Index::Position. Returns whether casting was necessary.
    fn fix_position(&mut self) -> Result<bool, BigFixedError> {
        let changed;
        match self.position {
            Index::Position(_) => {
                changed = false;
            },
            Index::Bit(b) => {
                let diff = Index::<D>::uncastsize(b.rem_euclid(Index::<D>::castsize(D::DIGITBITS)?))?; 
                let p = (b - Index::<D>::castsize(diff)?) / Index::<D>::castsize(D::DIGITBITS)?;
                // b = p*DIGITBITS + diff and 0 <= diff < DIGITBITS
                // shift left diff and change position to p
                if diff > 0 {
                    let opdiff = D::DIGITBITS - diff;
                    let keepmask: D = D::ALLONES >> diff;
                    let carrymask: D = !keepmask;
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
                    } else if self.is_neg()? {
                        self.body.push(((D::ALLONES & keepmask) << diff).into());
                    }
                }
                self.position = Index::Position(p);
                changed = true;
            },
            Index::DigitTypeInUse(_) => return Err(IndexError::UsedDigitTypeAsIndex.into())
        }
        Ok(changed)
    }

    fn construct(head: D, body: Vec<D>, position: Index<D>) -> Result<BigFixedVec<D>, BigFixedError> {
        let mut returner = BigFixedVec {
            head,
            body,
            position,
            zero_copy: D::ZERO,
            one_copy: D::ONE
        };
        returner.format()?;
        Ok(returner)
    }

    // Restructure if necessary so that all positions in low..high are valid. Breaks format so reformat afterwards. Returns whether restructuring was necessary.
    fn ensure_valid_range(&mut self, low: Index<D>, high: Index<D>) -> Result<bool, BigFixedError> {
        self.fix_position()?;
        if low >= high {
            if low == high {
                return Ok(false);
            } else {
                return self.ensure_valid_range(high, low);
            }
        }
        let low = low.cast_to_position()?;
        let high = high.cast_to_position()?;
        let shifted_low = (low - self.position)?;
        let shifted_high = (high - self.position)?;
        let add_low = (-shifted_low)?.unsigned_value()?;
        let add_high = (shifted_high - Index::<D>::castsize(self.body.len())?)?.unsigned_value()?;
        self.position = min(low, self.position);
        let reserve = add_low + add_high;
        if reserve > 0 {
            self.body.reserve(reserve);
            if add_low > 0 {
                self.body.splice(0..0, repeat(D::ZERO).take(add_low));
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
    fn ensure_valid_position(&mut self, position: Index<D>) -> Result<bool, BigFixedError> {
        let p = position.cast_to_position()?;
        self.ensure_valid_range(p, (p + Index::Position(1))?)
    }

    fn range_mut_iter(&mut self, low: Index<D>, high: Index<D>) -> Result<IterMut<D>, BigFixedError> {
        self.ensure_valid_range(low, high)?;
        Ok(self.body.iter_mut())
    }

    /*fn range_iter(&self, low: Index<D>, high: Index<D>) -> Result<<BigFixedVec<D> as BigFixed<D>>::BodyIterType, BigFixedError> {
        assert!(self.properly_positioned());
        let body_high = self.body_high()?;
        let low = low.cast_to_position()?;
        let keep_low = min(body_high, max(self.position, low));
        let keep_high = min(high, body_high);
        let high = high.cast_to_position()?;
        Ok(
            repeat(0.into()).take((self.position - low)?.unsigned_value()?)
            .chain(
                self.body.iter().map(D::deref)
                .skip((keep_low - self.position)?.unsigned_value()?)
                .take((keep_high - keep_low)?.unsigned_value()?)
            )
            .chain(
                repeat(self.head).take((high - body_high)?.unsigned_value()?)
            )
        )
    }*/

    fn is_neg(&self) -> Result<bool, BigFixedError> {
        Ok(self.head != D::ZERO)
    }

    // the least position which is outside of the range contained in body
    fn body_high(&self) -> Result<Index<D>, BigFixedError> {
        Ok((self.position + self.body.len())?)
    }

    fn valid_range(&self) -> Result<Range<Index<D>>, BigFixedError> {
        Ok(self.position..self.body_high()?)
    }

    fn int(&self) -> Result<BigFixedVec<D>, BigFixedError> {
        BigFixedVec::construct(
            self.head,
            self.body[(-self.position)?.unsigned_value()?..self.body.len()].to_vec(),
            self.position.saturating_nonnegative()?
        )
    }

    fn frac(&self) -> Result<BigFixedVec<D>, BigFixedError> {
        BigFixedVec::construct(
            D::ZERO,
            self.body[0..(-self.position)?.unsigned_value()?].to_vec(),
            self.position // if position is positive then body must be empty and format() resets position to 0
        )
    }

    fn overwrite(&mut self, src: &BigFixedVec<D>) {
        self.head = src.head();
        self.body.splice(0..self.body.len(), src.body.iter().map(|x| *x));
        self.position = src.position;
    }

    fn shift(mut self, shift: Index<D>) -> Result<BigFixedVec<D>, BigFixedError> {
        self.position += shift;
        self.format()?;
        Ok(self)
    }

    // does not require proper formatting -- fully checks if self is zero
    fn is_zero(&self) -> Result<bool, BigFixedError> {
        Ok(self.head == D::ZERO && self.body.iter().all(|&x| x == D::ZERO))
    }

    fn full_eq(&self, other: &BigFixedVec<D>) -> Result<bool, BigFixedError> {
        for i in min(
            self.position.cast_to_position()?,
            other.position.cast_to_position()?
        ).value()?..(
            max(
                self.body_high()?,
                other.body_high()?
            ) + Index::Position(1)
        )?.value()? {
            if self[Index::Position(i)] != other[Index::Position(i)] {
                return Ok(false);
            }
        };
        Ok(true)
    }

    fn cutoff_index(&self, cutoff: Cutoff<D>) -> Result<Index<D>, BigFixedError> {
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

    fn greatest_bit_position(&self) -> Result<Index<D>, BigFixedError> {
        // zero is special, just return 0
        if self.is_zero()? {
            return Ok(Index::Position(0));
        }
        let position = self.body_high()?;
        let coefficient: D = self[(position - 1isize)?] ^ self.head; // greatest bit which differs from head is greatest bit here
        Ok(Index::Bit(position.bit_value()? - Index::<D>::castsize(coefficient.value().leading_zeros() as usize + 1)?))
    }

    const ZERO: BigFixedVec<D> = BigFixedVec {
        head: D::ZERO,
        body: vec![],
        position: Index::Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
}

impl<D: Digit> CutsOff<D, BigFixedError> for BigFixedVec<D> {
    fn cutoff(&mut self, cutoff: Cutoff<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        let cutoff_index = self.cutoff_index(cutoff)?;
        let as_bit = cutoff_index.cast_to_bit()?;
        let as_pos = cutoff_index.cast_to_position()?;
        let increment = match cutoff.round {
            Rounding::Floor => false,
            Rounding::Round => self[(as_bit - Index::Bit(1))?] > D::ZERO,
            Rounding::Ceiling => {
                if self.position >= as_bit {
                    false
                } else {
                    let mut has_value = false;
                    for p in self.position.value()?..as_pos.value()? {
                        if self[Index::Position(p)] != D::ZERO {
                            has_value = true;
                            break;
                        }
                    }
                    let diff = as_bit.bit_position_excess()?;
                    if diff > 0 {
                        has_value = has_value || (self[as_pos] & (D::ALLONES >> (D::DIGITBITS as isize - diff as isize) as usize) > D::ZERO);
                    }
                    has_value
                }
            },
            Rounding::TowardsZero => {
                if self.is_neg()? {
                    return self.cutoff(
                        Cutoff {
                            fixed: cutoff.fixed,
                            floating: cutoff.floating,
                            round: Rounding::Floor
                        }
                    )
                } else {
                    return self.cutoff(
                        Cutoff {
                            fixed: cutoff.fixed,
                            floating: cutoff.floating,
                            round: Rounding::Ceiling
                        }
                    )
                }
            },
            Rounding::AwayFromZero => {
                if self.is_neg()? {
                    return self.cutoff(
                        Cutoff {
                            fixed: cutoff.fixed,
                            floating: cutoff.floating,
                            round: Rounding::Ceiling
                        }
                    )
                } else {
                    return self.cutoff(
                        Cutoff {
                            fixed: cutoff.fixed,
                            floating: cutoff.floating,
                            round: Rounding::Floor
                        }
                    )
                }
            }
        };
        if as_pos > self.position {
            self.body.drain(0..min(self.body.len(), (as_pos - self.position)?.into()));
            self.position = as_pos;
        }
        let diff = (as_bit - as_pos)?.unsigned_value()?;
        if diff > 0 {
            if self.body.len() == 0 {
                self.body.push(self.head);
            }
            self[as_pos] &= D::ALLONES << diff;
        }
        if increment {
            self.add_digit(D::ONE, as_bit)?;
        }
        self.format()?;
        Ok(())
    }
}

impl<D: Digit> fmt::Display for BigFixedVec<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_neg().unwrap() {
            return write!(f, "-{}", (-self).unwrap());
        };
        let (mut digits, mut point) = self.to_digits_10().unwrap();
        point = Index::<D>::castsize(digits.len()).unwrap() - point;
        digits.reverse();
        if point == 0 {
            write!(f, "0").ok();
        } else if point < 0 {
            write!(f, "0.{}", "0".repeat(Index::<D>::uncastsize(-point).unwrap())).ok();
        }
        for d in digits {
            if point == 0isize {
                write!(f, ".").ok();
            }
            point -= 1isize;
            write!(f, "{}", d).ok();
        }
        write!(f, "{}", "0".repeat(Index::<D>::saturating_unsigned(point).unwrap())).ok();
        write!(f, "")
    }
}

impl<D: Digit> fmt::Binary for BigFixedVec<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_neg().unwrap() {
            return write!(f, "-{:b}", (-self).unwrap());
        };
        let start = self.greatest_bit_position().unwrap().bit_value().unwrap();
        if start < 0 {
            write!(f, "{}", "0".repeat(Index::<D>::uncastsize(-start).unwrap())).ok();
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
