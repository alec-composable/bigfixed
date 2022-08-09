/*
    Cutting off is a two step process. First the cutoff index is determined. The cutoff index does not depend on sign or rounding,
    only on positional data. Second is to decide whether to round up or down. This depends on sign, the cutoff rounding technique, and
    the BigFixed itself.

    Once these two things have been determined, all data below the cutoff index is discarded (set to 0). If it rounds up then 1 is added
    in at the cutoff index.

    Cutoffs come in two kinds and how to compute the cutoff index is different for each. For fixed cutoffs the cutoff index is just the
    given fixed position. For floating cutoffs the cutoff index is the greatest bit position minus the given floating position.
*/

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
    // find where the cutoff should occur
    pub fn cutoff_index(&self, cutoff: Cutoff<D>) -> Result<Index<D>, BigFixedError> {
        match (cutoff.fixed, cutoff.floating) {
            (None, None) => Ok(self.position), // no cutoff
            (Some(fixed), None) => Ok(fixed),
            _ => self.cutoff_index_gb(cutoff, self.greatest_bit_position()?)
        }
    }

    // for shortcutting by not recomputing the greatest bit position
    pub(crate) fn cutoff_index_gb(&self, cutoff: Cutoff<D>, greatest_bit_position: Index<D>) -> Result<Index<D>, BigFixedError> {
        match (cutoff.fixed, cutoff.floating) {
            (None, None) => Ok(self.position), // no cutoff
            (Some(fixed), None) => Ok(fixed),
            (None, Some(floating)) => Ok((greatest_bit_position - floating)?),
            (Some(fixed), Some(floating)) => Ok(min(
                max(self.position, fixed),
                max(self.position, (greatest_bit_position - floating)?))
            )
        }
    }

    // rounding down means setting all data below the cutoff index to 0
    // rounding up means adding 1 to the cutoff index then setting all lower data to 0
    pub fn rounds_down(&self, cutoff: Cutoff<D>) -> Result<bool, BigFixedError> {
        return self.rounds_down_full(cutoff.round, self.cutoff_index(cutoff)?);
    }

    pub fn rounds_down_full(&self, cutoff_round: Rounding, cutoff_index: Index<D>) -> Result<bool, BigFixedError> {
        match cutoff_index {
            Index::Bit(_b) => {
                return Err(BigFixedError::ImproperlyPositioned);
            },
            Index::Position(_p) => {
                match cutoff_round {
                    Rounding::Floor => Ok(true),
                    Rounding::Ceiling => {
                        Ok(
                            self.body[
                                0 .. min(self.body.len(), (cutoff_index - self.position)?.unsigned_value()?)
                            ].iter().all(
                                |&x| x == D::ZERO
                            )
                        )
                    },
                    Rounding::Round => Ok(
                        *self.index_result((cutoff_index - Index::Position(1))?)? < D::GREATESTBIT
                    ),
                    Rounding::TowardsZero => {
                        if self.is_neg() {
                            return self.rounds_down_full(Rounding::Ceiling, cutoff_index);
                        } else {
                            return self.rounds_down_full(Rounding::Floor, cutoff_index);
                        }
                    },
                    Rounding::AwayFromZero => {
                        if self.is_neg() {
                            return self.rounds_down_full(Rounding::Floor, cutoff_index);
                        } else {
                            return self.rounds_down_full(Rounding::Ceiling, cutoff_index);
                        }
                    }
                }
            },
            Index::DigitTypeInUse(_) => Err(BigFixedError::UNINDEXED_INDEX)
        }
    }
}

impl<D: Digit> CutsOff<D, BigFixedError> for BigFixed<D> {
    fn cutoff(&mut self, cutoff: Cutoff<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        let cutoff_index = self.cutoff_index(cutoff)?;
        let as_bit = cutoff_index.cast_to_bit()?;
        let as_pos = cutoff_index.cast_to_position()?;
        let increment = !self.rounds_down(cutoff)?;
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
