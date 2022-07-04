use crate::{digit::*, Index as Indx, BigFixed, BigFixedVec};

use std::{ops::{Index, IndexMut}};

impl<D: Digit> Index<Indx<D>> for BigFixedVec<D> {
    type Output = D;
    fn index(&self, position: Indx<D>) -> &D {
        assert!(self.properly_positioned(), "indexing into an impoperly positioned BigFixed");
        match position {
            Indx::Position(_) => {
                let shifted = (position - self.position).unwrap();
                if shifted >= self.body.len() as isize {
                    &self.head
                } else if shifted >= 0isize {
                    &self.body[usize::from(shifted)]
                } else {
                    &self.zero_copy
                }
            },
            Indx::Bit(b) => {
                let d = self[Indx::Position(Indx::<D>::bit_to_position(b))];
                if (d >> position.bit_position_excess().unwrap()) & D::ONE == D::ONE {
                    &self.one_copy
                } else {
                    &self.zero_copy
                }
            },
            Indx::DigitTypeInUse(_) => panic!("cannot properly position")
        }
    }
}

impl<D: Digit> Index<isize> for BigFixedVec<D> {
    type Output = D;
    fn index(&self, position: isize) -> &D {
        &self[Indx::Position(position)]
    }
}

// Gives a reference to the digit in the corresponding position regardless of Index type (Bit/Position). Use set_bit for bit-level mutation.
impl<D: Digit> IndexMut<Indx<D>> for BigFixedVec<D> {
    fn index_mut(&mut self, position: Indx<D>) -> &mut D {
        let position = position.cast_to_position().unwrap();
        self.ensure_valid_position(position).unwrap(); // includes a call to self.fix_position();
        self.body.index_mut(usize::from((position - self.position).unwrap()))
    }
}

impl<D: Digit> IndexMut<isize> for BigFixedVec<D> {
    fn index_mut(&mut self, position: isize) -> &mut D {
        self.index_mut(Indx::Position(position))
    }
}

impl<D: Digit> BigFixedVec<D> {
    pub fn set_bit(&mut self, index: isize, value: D) {
        assert!(value == D::ZERO || value == D::ONE, "set_bit requires a bit (0 or 1)");
        let position = Indx::Bit(index);
        let shift = Indx::bit_position_excess(&position).unwrap();
        self[position] = (self[position.cast_to_position().unwrap()] & !(D::ONE << shift)) | (value << shift);
    }
}
