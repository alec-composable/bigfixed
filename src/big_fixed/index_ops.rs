use crate::{digit::*, Index as Indx, BigFixed};

use std::{ops::{Index, IndexMut}};

impl Index<Indx> for BigFixed {
    type Output = Digit;
    fn index(&self, position: Indx) -> &Digit {
        assert!(self.properly_positioned());
        let shifted = (position.cast_to_position() - self.position).unwrap();
        if shifted >= self.body.len() as isize {
            &self.head
        } else if shifted >= 0isize {
            &self.body[usize::from(shifted)]
        } else {
            &0
        }
    }
}

impl Index<isize> for BigFixed {
    type Output = Digit;
    fn index(&self, position: isize) -> &Digit {
        &self[Indx::Position(position)]
    }
}

impl IndexMut<Indx> for BigFixed {
    fn index_mut(&mut self, position: Indx) -> &mut Digit {
        let position = position.cast_to_position();
        self.ensure_valid_position(position).unwrap();
        self.body.index_mut(usize::from((position - self.position).unwrap()))
    }
}

impl IndexMut<isize> for BigFixed {
    fn index_mut(&mut self, position: isize) -> &mut Digit {
        self.index_mut(Indx::Position(position))
    }
}
