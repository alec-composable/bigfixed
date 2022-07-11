use crate::{digit::*, Index, IndexError/*, cutoff::**/};

use core::{
    fmt,
    convert
};

use std::error::Error;

pub mod position;
pub mod range;
pub mod rearrangements;
pub mod ops {
    pub mod index;
}
//pub mod convert;
//pub mod ops;
//pub mod ops_c;
//pub mod exp;


#[derive(Clone, Copy, Debug)]
pub enum BigFixedError {
    // absorb IndexErrors
    IndexError(IndexError),
    // ImproperlyPositioned is for when self.position is Index::Bit when Index::Position was expected
    ImproperlyPositioned,
    // not applicable to Vec-based BigFixed but potentially applicable to hard coded versions
    OutOfBoundsError
}

impl convert::From<IndexError> for BigFixedError {
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

pub trait BigFixedErrorReferences: 'static {
    const IMPROPERLYPOSITIONED_R: &'static Result<(), BigFixedError> = &Err(BigFixedError::ImproperlyPositioned);
    const OUTOFBOUNDSERROR_R: &'static Result<(), BigFixedError> = &Err(BigFixedError::OutOfBoundsError);
}

impl BigFixedErrorReferences for BigFixedError {}

pub trait DigitIterator<D: Digit>:
    Iterator<Item = D>
    + DoubleEndedIterator
{}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BigFixed<D: Digit> {
    pub head: D,
    pub body: Vec<D>,
    pub position: Index<D>
}

impl<D: Digit> BigFixed<D> {
    pub fn is_neg(&self) -> bool {
        self.head != D::ZERO
    }
}

/*impl<D: Digit> BigFixed<D> {
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
}*/

impl<D: Digit> fmt::Binary for BigFixed<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "head: {:b}", self.head).ok();
        for x in self.body.iter().rev() {
            write!(f, " {:b}", x).ok();
        }
        write!(f, " position {}", self.position)
    }
}
