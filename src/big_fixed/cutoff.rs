use crate::{
    Digit,
    Index,
    Cutoff,
    Rounding,
    CutsOff,
    BigFixed,
    BigFixedError
};

use core::{
    cmp::{
        max,
        min
    }
};

impl<D: Digit> BigFixed<D> {
    pub fn cutoff_index(&self, cutoff: Cutoff<D>) -> Result<Index<D>, BigFixedError> {
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
}

impl<D: Digit> CutsOff<D, BigFixedError> for BigFixed<D> {
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
                if self.is_neg() {
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
                if self.is_neg() {
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
            self.body.drain(0..min(self.body.len(), (as_pos - self.position)?.unsigned_value()?));
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
