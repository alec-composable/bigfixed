/*
    Cutoff operations for BigFixed. Essentially the process is to do the pure operation then cut off the result. Later on we can implement an optimization
    to avoid computing data which is just going to be cut off, but that was slightly more complicated than it first seemed so this first attempt takes
    the simplest approach.
*/

use crate::{digit::*, Index as Indx, Cutoff, CutsOff, BigFixed, macros::*};

use std::{ops::*};


impl BigFixed {
    pub fn shift_c(mut self, shift: isize, cutoff: Cutoff) -> BigFixed {
        self = self.shift(shift);
        self.cutoff(cutoff);
        self
    }

    // Add digit into position and handle carries
    pub fn add_digit_c(&mut self, d: Digit, position: Indx, cutoff: Cutoff) {
        if position < self.cutoff_position(cutoff) {return};
        let mut res;
        let mut carry;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.body_high();
        let mut on_position = position + 1isize;
        while carry == 1 && on_position < high {
            add!(self[on_position], 1, res, carry);
            self[on_position] = res;
            on_position += 1isize;
        }
        // overflow cases
        if carry == 1 {
            if self.is_neg() {
                self.head = 0;
            } else {
                self[high] = 1;
            }
        }
    }

    pub fn add_digit_drop_overflow_c(&mut self, d: Digit, position: Indx, cutoff: Cutoff) {
        if position < self.cutoff_position(cutoff) {return};
        if position >= self.body_high() {
            // already overflows
            return;
        }
        let mut res;
        let mut carry;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.body_high();
        let mut on_position = position + 1isize;
        while carry == 1 && on_position < high {
            add!(self[on_position], 1, res, carry);
            self[on_position] = res;
            on_position += 1isize;
        }
    }
}

cutoff_op!(Add, add, AddAssign, add_assign, BigFixed, BigFixed, BigFixed);
cutoff_op!(BitAnd, bitand, BitAndAssign, bitand_assign, BigFixed, BigFixed, BigFixed);
cutoff_op!(BitOr, bitor, BitOrAssign, bitor_assign,BigFixed, BigFixed, BigFixed);
cutoff_op!(BitXor, bitxor, BitXorAssign, bitxor_assign, BigFixed, BigFixed, BigFixed);
cutoff_op!(Mul, mul, MulAssign, mul_assign, BigFixed, BigFixed, BigFixed);
//unary!(Neg, neg, BigFixed); -- not sure what the best aproach is for unary operations
//unary!(Not, not, BigFixed);
cutoff_op!(Shl, shl, ShlAssign, shl_assign, BigFixed, usize, BigFixed);
cutoff_op!(Shr, shr, ShrAssign, shr_assign, BigFixed, usize, BigFixed);
cutoff_op!(Sub, sub, SubAssign, sub_assign, BigFixed, BigFixed, BigFixed);
