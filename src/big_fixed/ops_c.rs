/*
    Cutoff operations for BigFixed. Essentially the process is to do the pure operation then cut off the result. Later on we can implement an optimization
    to avoid computing data which is just going to be cut off but for now we do the easy way
*/

use crate::{digit::*, Index, Cutoff, cutoff::*, BigFixed, BigFixedError, macros::*};

use std::{
    ops::{
        Add, AddAssign,
        BitAnd, BitAndAssign,
        BitOr, BitOrAssign,
        BitXor, BitXorAssign,
        Mul, MulAssign,
        Shl, ShlAssign,
        Shr, ShrAssign,
        Sub, SubAssign
    },
    cmp::{Ordering}
};

impl BigFixed {
    pub fn format_c(&mut self, cutoff: Cutoff) -> Result<(), BigFixedError> {
        self.cutoff(cutoff)?;
        self.format()
    }

    pub fn shift_c(mut self, shift: Index, cutoff: Cutoff) -> Result<BigFixed, BigFixedError> {
        self = self.shift(shift)?;
        self.cutoff(cutoff)?;
        Ok(self)
    }

    pub fn is_zero_c(&self, cutoff: Cutoff) -> Result<bool, BigFixedError> {
        let mut avoidable_clone = self.clone();
        avoidable_clone.cutoff(cutoff)?;
        Ok(avoidable_clone.is_zero())
    }

    pub fn full_eq_c(&self, other: &BigFixed, cutoff: Cutoff) -> Result<bool, BigFixedError> {
        let mut avoidable_clone_1 = self.clone();
        let mut avoidable_clone_2 = other.clone();
        avoidable_clone_1.cutoff(cutoff)?;
        avoidable_clone_2.cutoff(cutoff)?;
        avoidable_clone_1.full_eq(&avoidable_clone_2)
    }

    pub fn cutoff_index_c(&self, cutoff_c: Cutoff, cutoff: Cutoff) -> Result<Index, BigFixedError> {
        let mut avoidable_clone = self.clone();
        avoidable_clone.cutoff(cutoff)?;
        avoidable_clone.cutoff_index(cutoff_c)
    }

    pub fn greatest_bit_position_c(&self, cutoff: Cutoff) -> Result<Index, BigFixedError> {
        let mut avoidable_clone = self.clone();
        avoidable_clone.cutoff(cutoff)?;
        avoidable_clone.greatest_bit_position()
    }

    // Add digit into position and handle carries
    pub fn add_digit_c(&mut self, d: Digit, position: Index, cutoff: Cutoff) -> Result<(), BigFixedError> {
        self.add_digit(d, position)?;
        self.cutoff(cutoff)
    }

    // add_digit but leaves (positionally entire) head unchanged
    pub fn add_digit_drop_overflow_c(&mut self, d: Digit, position: Index, cutoff: Cutoff) -> Result<(), BigFixedError> {
        self.add_digit_drop_overflow(d, position)?;
        self.cutoff(cutoff)
    }

    pub fn abs_c(&self, cutoff: Cutoff) -> Result<BigFixed, BigFixedError> {
        let mut res = self.abs()?;
        res.cutoff(cutoff)?;
        Ok(res)
    }
}

/*
op_assign_to_op!(op, op_fn_name, op_assign, op_assign_fn_name, op_assign_c_fn_name, self_type, other_type, cutoff_type, result_type, error_type)
*/
cutoff_op!(Add, add, add_c, AddAssign, add_assign, add_assign_c, BigFixed, BigFixed, Cutoff, cutoff, BigFixed, BigFixedError);
cutoff_op!(BitAnd, bitand, bitand_c, BitAndAssign, bitand_assign, bitand_assign_c, BigFixed, BigFixed, Cutoff, cutoff, BigFixed, BigFixedError);
cutoff_op!(BitOr, bitor, bitor_c, BitOrAssign, bitor_assign, bitor_assign_c, BigFixed, BigFixed, Cutoff, cutoff, BigFixed, BigFixedError);
cutoff_op!(BitXor, bitxor, bitxor_c, BitXorAssign, bitxor_assign, bitxor_assign_c, BigFixed, BigFixed, Cutoff, cutoff, BigFixed, BigFixedError);
cutoff_op!(Mul, mul, mul_c, MulAssign, mul_assign, mul_assign_c, BigFixed, BigFixed, Cutoff, cutoff, BigFixed, BigFixedError);
cutoff_op!(Shl, shl, shl_c, ShlAssign, shl_assign, shl_assign_c, BigFixed, usize, Cutoff, cutoff, BigFixed, BigFixedError);
cutoff_op!(Shr, shr, shr_c, ShrAssign, shr_assign, shr_assign_c, BigFixed, usize, Cutoff, cutoff, BigFixed, BigFixedError);
cutoff_op!(Sub, sub, sub_c, SubAssign, sub_assign, sub_assign_c, BigFixed, BigFixed, Cutoff, cutoff, BigFixed, BigFixedError);

cutoff_op!(Neg, neg, BigFixed, negate, negate_c, Cutoff, cutoff, BigFixedError);

// Rem and RemAssign depend on division

impl BigFixed {
    // combined_div

    pub fn to_digits_c(&self, base: &BigFixed, max_len: usize) -> Result<(Vec<BigFixed>, isize), BigFixedError> {
        let mut shifting = self.abs()?.clone();
        let mut neg_count: isize = 0;
        while shifting.position < 0isize {
            shifting *= base;
            neg_count += 1;
        }
        let mut digits = vec![];
        while !shifting.is_zero() {
            let quot = BigFixed::combined_div(&mut shifting, base, 0)?;
            digits.push(shifting.clone());
            shifting = quot;
        }
        if digits.len() > max_len {
            let drain = digits.len() - max_len;
            neg_count -= Index::castsize(drain)?;
            digits.drain(0..drain);
        }
        Ok((digits, neg_count))
    }

    pub fn to_digits_10_c(&self, max_len: usize) -> Result<(Vec<i32>, isize), BigFixedError> {
        let (big_digits, point) = self.to_digits_c(&BigFixed::from(10), max_len)?;
        let digits: Vec<i32> = big_digits.iter().map(|x| i32::from(x)).collect();
        Ok((digits, point))
    }
}

impl BigFixed {
    pub fn partial_cmp_c(&self, other: &BigFixed, cutoff: Cutoff) -> Option<Ordering> {
        let mut avoidable_clone_1 = self.clone();
        let mut avoidable_clone_2 = other.clone();
        avoidable_clone_1.cutoff(cutoff).ok()?;
        avoidable_clone_2.cutoff(cutoff).ok()?;
        avoidable_clone_1.partial_cmp(&avoidable_clone_2)
    }
}

