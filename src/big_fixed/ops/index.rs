/*
    The std ops Index and IndexMut apply to BigFixeds wrt bigfixed::Index, giving the corresponding digit.
    Index returns either the head, body element, or tail depending on position. Index does not alter the BigFixed.
    IndexMut first calls ensure_valid_position to ensure the position is in the body then returns the corresponding &mut D reference.
    Both have possible errors from bigfixed::Index stuff so both have a result version as well. The std::ops version takes this
    result version and then unwraps it.While succinct and convenient to use, it is advised to replace any x[...] operations
    with the corresponding result version in the final product for more proper error handling.
*/

use crate::{
    Digit,
    Index as Indx,
    IndexError,
    BigFixed,
    BigFixedError
};

use std::{
    ops::{
        Index,
        IndexMut
    }
};

impl<D: Digit> BigFixed<D> {
    pub fn index_result(&self, position: Indx<D>) -> Result<&D, BigFixedError> {
        self.properly_positioned_screen()?;
        match position {
            Indx::Position(_) => {
                let shifted = (position - self.position)?;
                if shifted >= self.body.len() as isize {
                    Ok(&self.head)
                } else if shifted >= 0isize {
                    Ok(&self.body[shifted.unsigned_value()?])
                } else {
                    Ok(D::ZERO_R)
                }
            },
            Indx::Bit(b) => {
                let d = self.index_result(Indx::Position(Indx::<D>::bit_to_position(b)))?;
                if (*d >> position.bit_position_excess()?) & D::ONE == D::ONE {
                    Ok(D::ONE_R)
                } else {
                    Ok(D::ZERO_R)
                }
            },
            Indx::DigitTypeInUse(_) => Err(BigFixedError::IndexError(IndexError::UsedDigitTypeAsIndex))
        }
    }

    pub fn index_mut_result(&mut self, position: Indx<D>) -> Result<&mut D, BigFixedError> {
        let position = position.cast_to_position()?;
        self.ensure_valid_position(position)?; // includes a call to self.fix_position();
        Ok(self.body.index_mut((position - self.position)?.unsigned_value()?))
    }
}

impl<D: Digit> Index<Indx<D>> for BigFixed<D> {
    type Output = D;
    fn index(&self, position: Indx<D>) -> &D {
        &self.index_result(position).unwrap()
    }
}

impl<D: Digit> Index<isize> for BigFixed<D> {
    type Output = D;
    fn index(&self, position: isize) -> &D {
        &self[Indx::Position(position)]
    }
}

impl<D: Digit> Index<usize> for BigFixed<D> {
    type Output = D;
    fn index(&self, position: usize) -> &D {
        &self[Indx::Position(Indx::<D>::castsize(position).unwrap())]
    }
}

// Gives a reference to the digit in the corresponding position regardless of Index type (Bit/Position). Use set_bit for bit-level mutation.
impl<D: Digit> IndexMut<Indx<D>> for BigFixed<D> {
    fn index_mut(&mut self, position: Indx<D>) -> &mut D {
        self.index_mut_result(position).unwrap()
    }
}

impl<D: Digit> IndexMut<isize> for BigFixed<D> {
    fn index_mut(&mut self, position: isize) -> &mut D {
        self.index_mut(Indx::Position(position))
    }
}

impl<D: Digit> IndexMut<usize> for BigFixed<D> {
    fn index_mut(&mut self, position: usize) -> &mut D {
        self.index_mut(Indx::Position(Indx::<D>::castsize(position).unwrap()))
    }
}

impl<D: Digit> BigFixed<D> {
    pub fn set_bit(&mut self, index: isize, value: bool) -> Result<(), BigFixedError> {
        let position = Indx::Bit(index);
        let bit = if value {
            D::ONE
        } else {
            D::ZERO
        };
        let shift = Indx::bit_position_excess(&position)?;
        self[position] = (self[position.cast_to_position()?] & !(D::ONE << shift)) | (bit << shift);
        Ok(())
    }
}
