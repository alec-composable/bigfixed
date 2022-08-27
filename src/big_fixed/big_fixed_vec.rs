use crate::{digit::*, Index, BigFixed};

use std::vec::IntoIter;

use core::{
    fmt
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BigFixedVec<D: Digit> {
    pub head: D,
    pub body: Vec<D>,
    pub position: Index<D>
}

impl<D: Digit> BigFixed<D> for BigFixedVec<D> {
    fn head(&self) -> D {
        self.head
    }

    fn position(&self) -> Index<D> {
        self.position
    }

    type BodyIter = IntoIter<D>;

    fn body_iter(&self) -> Self::BodyIter {
        self.body.clone().into_iter()
    }

    const ZERO: Self = BigFixedVec {
        head: D::ZERO,
        body: vec![],
        position: Index::Position(0)
    };
}

impl<D: Digit> fmt::Binary for BigFixedVec<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_binary(f)
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
