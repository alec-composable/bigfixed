use crate::{
    Digit,
    Index,
    Rounding,
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
        Skip,
        Rev
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
        >,
        BigFixedError
    > {
        self.properly_positioned_screen()?;
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

    pub fn range_iter_rev(&self, low: Index<D>, high: Index<D>) -> Result<
        Chain<
            Chain<
                Take<Repeat<&D>>,
                Rev<Take<Skip<Iter<D>>>>
            >,
            Take<Repeat<&D>>
        >,
        BigFixedError
    > {
        self.properly_positioned_screen()?;
        let body_high = self.body_high()?;
        let low = low.cast_to_position()?;
        let keep_low = min(body_high, max(self.position, low));
        let keep_high = min(high, body_high);
        let high = high.cast_to_position()?;
        Ok(
            repeat(&self.head).take((high - body_high)?.unsigned_value()?)
            .chain(
                self.body.iter()
                .skip((keep_low - self.position)?.unsigned_value()?)
                .take((keep_high - keep_low)?.unsigned_value()?)
                .rev()
            )
            .chain(
                repeat(D::ZERO_R).take((self.position - low)?.unsigned_value()?)
            )
        )
    }

    pub fn range_iter_cutoff_full(&self, round: Rounding, cutoff_position: Index<D>, low: Index<D>, high: Index<D>) -> Result<
        Take<Repeat<&D>>
    , BigFixedError> {
        self.properly_positioned_screen()?;
        if self.rounds_down_full(round, cutoff_position)? {
            Ok(
                repeat(D::ZERO_R).take((low - cutoff_position)?.unsigned_value()?)
            )
        } else {
            panic!("cannot round up yet");
        }
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

    pub fn full_eq(&self, other: &BigFixed<D>) -> Result<bool, BigFixedError> {
        let min = min(
            self.position.cast_to_position()?,
            other.position.cast_to_position()?
        );
        let max = max(
            self.body_high()?,
            other.body_high()?
        );
        let me = self.range_iter(min, max)?;
        let you = other.range_iter(min, max)?;
        for (x, y) in me.zip(you) {
            if *x != *y {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
