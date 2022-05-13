use crate::{digit::*, Index as Indx, BigFixed};

use std::{ops::{Index, IndexMut}};

impl Index<Indx> for BigFixed {
    type Output = Digit;
    fn index(&self, position: Indx) -> &Digit {
        assert!(self.properly_positioned(), "indexing into an impoperly positioned BigFixed");
        match position {
            Indx::Position(_) => {
                let shifted = (position - self.position).unwrap();
                if shifted >= self.body.len() as isize {
                    &self.head
                } else if shifted >= 0isize {
                    &self.body[usize::from(shifted)]
                } else {
                    &0
                }
            },
            Indx::Bit(b) => {
                let d = self[Indx::bit_to_position(b)];
                if (d >> position.bit_position_excess()) & 1 == 1 {
                    &1
                } else {
                    &0
                }
            }
        }
    }
}

impl Index<isize> for BigFixed {
    type Output = Digit;
    fn index(&self, position: isize) -> &Digit {
        &self[Indx::Position(position)]
    }
}

// Gives a reference to the digit in the corresponding position regardless of Index type (Bit/Position). Use set_bit for bit-level mutation.
impl IndexMut<Indx> for BigFixed {
    fn index_mut(&mut self, position: Indx) -> &mut Digit {
        let position = position.cast_to_position();
        self.ensure_valid_position(position).unwrap(); // includes a call to self.fix_position();
        self.body.index_mut(usize::from((position - self.position).unwrap()))
    }
}

impl IndexMut<isize> for BigFixed {
    fn index_mut(&mut self, position: isize) -> &mut Digit {
        self.index_mut(Indx::Position(position))
    }
}

impl BigFixed {
    pub fn set_bit(&mut self, index: isize, value: Digit) {
        assert!(value == 0 || value == 1, "set_bit requires a bit (0 or 1)");
        let position = Indx::Bit(index);
        let shift = Indx::bit_position_excess(&position);
        self[position] = (self[position.cast_to_position()] & !(1 << shift)) | (value << shift);
    }
}
