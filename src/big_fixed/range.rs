use crate::{
    Digit,
    Index,
    BigFixed,
    BigFixedError
};

use core::{
    cmp::{
        max,
        min
    },
    iter::{
        repeat,
        Take,
        Repeat,
        Chain,
        Skip
    },
    slice::{
        IterMut,
        Iter
    },
    ops::{
        Range
    }
};

impl<D: Digit> BigFixed<D> {
    // the least position which is outside of the range contained in body
    pub fn body_high(&self) -> Result<Index<D>, BigFixedError> {
        Ok((self.position + Index::Position(Index::<D>::castsize(self.body.len())?))?)
    }

    // Restructure if necessary so that all positions in low..high are valid. Breaks format so reformat afterwards. Returns whether restructuring was necessary.
    pub fn ensure_valid_range(&mut self, low: Index<D>, high: Index<D>) -> Result<bool, BigFixedError> {
        if low >= high {
            if low == high {
                self.fix_position()?;
                return Ok(false);
            } else {
                return self.ensure_valid_range(high, low);
            }
        }
        self.fix_position()?;
        let low = low.cast_to_position()?;
        let high = high.cast_to_position()?;
        let shifted_low = (low - self.position)?;
        let shifted_high = (high - self.position)?;
        let add_low = (-shifted_low)?.unsigned_value()?;
        let add_high = (shifted_high - self.body.len())?.unsigned_value()?;
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
    pub fn ensure_valid_position(&mut self, position: Index<D>) -> Result<bool, BigFixedError> {
        let p = position.cast_to_position()?;
        self.ensure_valid_range(p, (p + Index::Position(1))?)
    }

    pub fn range_mut_iter(&mut self, low: Index<D>, high: Index<D>) -> Result<IterMut<D>, BigFixedError> {
        self.ensure_valid_range(low, high)?;
        Ok(self.body.iter_mut())
    }

    pub fn range_iter(&self, low: Index<D>, high: Index<D>) -> Result<
        Chain<
            Chain<
                Take<Repeat<&D>>,
                Take<Skip<Iter<D>>>
            >, Take<Repeat<&D>>
        > , BigFixedError> {
        self.properly_positioned_result()?;
        let body_high = self.body_high()?;
        let low = low.cast_to_position()?;
        let keep_low = min(body_high, max(self.position, low));
        let keep_high = min(high, body_high);
        let high = high.cast_to_position()?;
        Ok(
            repeat(D::ZERO_R).take((self.position - low)?.unsigned_value()?)
            .chain(
                self.body.iter()
                .skip((keep_low - self.position)?.unsigned_value()?)
                .take((keep_high - keep_low)?.unsigned_value()?)
            )
            .chain(
                repeat(&self.head).take((high - body_high)?.unsigned_value()?)
            )
        )
    }

    pub fn valid_range(&self) -> Result<Range<Index<D>>, BigFixedError> {
        Ok(self.position..self.body_high()?)
    }

    pub fn shift(mut self, shift: Index<D>) -> Result<BigFixed<D>, BigFixedError> {
        self.position += shift;
        self.format()?;
        Ok(self)
    }

    pub fn overwrite(&mut self, src: &BigFixed<D>) {
        self.head = src.head;
        self.body.splice(0..self.body.len(), src.body.iter().map(|x| *x));
        self.position = src.position;
    }
}
