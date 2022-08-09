use crate::{
    Digit,
    Index,
    Cutoff,
    CutsOff,
    Rounding,
    BigFixed,
    BigFixedError
};

use core::{
    //ops::{
    //    Add, AddAssign
    //},
    cmp::{
        max, min
    }
};

impl<D: Digit> BigFixed<D> {
    // Add digit into position and handle carries
    pub fn add_digit(&mut self, d: D, position: Index<D>) -> Result<(), BigFixedError> {
        self.properly_positioned_screen()?;
        match position {
            Index::Position(_) => {
                self.ensure_valid_position(position)?;
                let mut writer = self.body.iter_mut().skip(
                    (position - self.position)?.unsigned_value()?
                );
                let mut carry = d;
                loop {
                    match writer.next() {
                        Some(x) => {
                            D::combined_add(*x, carry, x, &mut carry);
                            if carry == D::ZERO {
                                break;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }

                // overflow cases
                if carry == D::ONE {
                    if self.is_neg() {
                        self.head = D::ZERO;
                    } else {
                        self.body.push(D::ONE);
                    }
                };
                Ok(())
            },
            Index::Bit(_) => {
                let diff = position.bit_position_excess()?;
                if diff == 0 {
                    return self.add_digit(d, position.cast_to_position()?);
                }
                let as_position = position.cast_to_position()?;
                self.add_digit(d >> (D::DIGITBITS - diff), (as_position + Index::Position(1))?)?;
                self.add_digit(d << diff, as_position)?;
                return Ok(())
            },
            Index::DigitTypeInUse(_) => {
                return Err(BigFixedError::UNINDEXED_INDEX)
            }
        }
    }

    pub fn increment(&mut self) -> Result<(), BigFixedError> {
        self.add_digit(D::ONE, Index::Position(0))
    }

    // add_digit but leaves (positionally entire) head unchanged
    pub fn add_digit_drop_overflow(&mut self, d: D, position: Index<D>) -> Result<(), BigFixedError> {
        if position >= self.body_high()? {
            return Ok(());
        }
        self.properly_positioned_screen()?;
        match position {
            Index::Position(_) => {
                self.ensure_valid_position(position)?;
                let mut writer = self.body.iter_mut().skip(
                    (position - self.position)?.unsigned_value()?
                );
                let mut carry = d;
                loop {
                    match writer.next() {
                        Some(x) => {
                            D::combined_add(*x, carry, x, &mut carry);
                            if carry == D::ZERO {
                                break;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                Ok(())
            },
            Index::Bit(_) => {
                let diff = position.bit_position_excess()?;
                if diff == 0 {
                    return self.add_digit(d, position.cast_to_position()?);
                }
                let as_position = position.cast_to_position()?;
                self.add_digit(d >> (D::DIGITBITS - diff), (as_position + Index::Position(1))?)?;
                self.add_digit(d << diff, as_position)?;
                return Ok(())
            },
            Index::DigitTypeInUse(_) => {
                return Err(BigFixedError::UNINDEXED_INDEX)
            }
        }
    }

    // mutate in place to negative
    pub fn negate(&mut self) -> Result<(), BigFixedError> {
        self.head = !self.head;
        self.body.iter_mut().for_each(|x| *x = !*x);
        self.add_digit(D::ONE, self.position)?;
        self.format()?;
        Ok(())
    }

    pub fn abs(&self) -> Result<BigFixed<D>, BigFixedError> {
        let mut abs = self.clone();
        if self.is_neg() {
            abs.negate()?;
        }
        Ok(abs)
    }

    pub fn add_assign_res(&mut self, other: &BigFixed<D>, cutoff: Cutoff<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        other.properly_positioned_screen()?;
        let low = min(self.position, other.position);
        let high = max(self.body_high()?, other.body_high()?);
        self.ensure_valid_range(low, high)?;
        let mut read_other = other.range_iter_rev(low, high)?;
        match cutoff.round {
            Rounding::Floor | Rounding::Round => {
                let mut on_position = high;
                loop {
                    on_position = (on_position - Index::Position(1))?;
                    match read_other.next() {
                        Some(&d) => {
                            self.add_digit_drop_overflow(d, on_position)?;
                        },
                        None => {
                            break;
                        }
                    }
                }
            },
            Rounding::Ceiling => {
                panic!("not supported yet");
            },
            Rounding::TowardsZero | Rounding::AwayFromZero => {
                panic!("not supported yet");
                // have to estimate whether result is positive or negative
            }
        }
        self.cutoff(cutoff)?;
        Ok(())
    }

    /*pub fn add_assign(&mut self, other: &BigFixedVec<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        let position = min(self.position, other.position);
        // one more for overflow
        let high = (max(self.body_high()?, other.body_high()?) + Index::Position(1))?;
        self.ensure_valid_range(position, high)?;
        let other_low = other.position.cast_to_position()?;
        for i in other_low.value()?..high.value()? {
            let p = Index::Position(i);
            self.add_digit_drop_overflow(other[p], p)?;
        }
        self.head = if self[(high - Index::Position(1))?] >= D::GREATESTBIT {
            D::ALLONES
        } else {
            D::ZERO
        };
        self.format()
    }*/
}