use crate::{
    Digit,
    Index,
    BigFixed,
    BigFixedError
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
        for i in 0..self.body.len() {
            self.body[i] = !self.body[i];
        }
        self.add_digit(D::ONE, self.position)?;
        self.format()?;
        Ok(())
    }

    pub fn abs(&self) -> Result<BigFixed<D>, BigFixedError> {
        if self.is_neg() {
            let mut copy = self.clone();
            copy.negate()?;
            Ok(copy)
        } else {
            Ok(self.clone())
        }
    }
}