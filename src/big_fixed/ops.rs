use crate::{digit::*, Index, Cutoff, CutsOff, BigFixed, BigFixedVec, BigFixedError, macros::*};

use std::{
    ops::{
        Add, AddAssign,
        BitAnd, BitAndAssign,
        BitOr, BitOrAssign,
        BitXor, BitXorAssign,
        Mul, MulAssign,
        Neg, Not,
        Shl, ShlAssign,
        Shr, ShrAssign,
        Sub, SubAssign
    },
    cmp::{max, min, Ordering}
};

impl<D: Digit> BigFixedVec<D> {
    // Add digit into position and handle carries
    pub fn add_digit(&mut self, d: D, position: Index<D>) -> Result<(), BigFixedError> {
        assert!(self.properly_positioned());
        if let Index::Bit(_) = position {
            let diff = position.bit_position_excess()?;
            if diff == 0 {
                return self.add_digit(d, position.cast_to_position()?);
            }
            let as_position = position.cast_to_position()?;
            self.add_digit(d >> (D::DIGITBITS as usize - diff), (as_position + Index::Position(1))?)?;
            self.add_digit(d << diff, as_position)?;
            return Ok(())
        }
        self.ensure_valid_position(position)?;
        let mut carry = D::ZERO;
        D::add_full(self[position], d, &mut self[position], &mut carry);
        let high = self.body_high()?;
        let mut on_position = (position + 1isize)?;
        while carry == D::ONE && on_position < high {
            D::add_full(self[on_position], D::ONE, &mut self[on_position], &mut carry);
            on_position += Index::Position(1);
        }
        // overflow cases
        if carry == D::ONE {
            if self.is_neg()? {
                self.head = D::ZERO;
            } else {
                self[high] = D::ONE;
            }
        };
        Ok(())
    }

    pub fn increment(&mut self) -> Result<(), BigFixedError> {
        self.add_digit(D::ONE, Index::Position(0))
    }

    // add_digit but leaves (positionally entire) head unchanged
    pub fn add_digit_drop_overflow(&mut self, d: D, position: Index<D>) -> Result<(), BigFixedError> {
        assert!(self.properly_positioned());
        if position >= self.body_high()? {
            // already overflows
            return Ok(());
        }
        if let Index::Bit(_b) = position {
            if let Index::Bit(_) = position {
                let diff = position.bit_position_excess()?;
                let as_position = position.cast_to_position()?;
                self.add_digit_drop_overflow(d >> (D::DIGITBITS as usize - diff), (as_position + Index::Position(1))?)?;
                self.add_digit_drop_overflow(d << diff, as_position)?;
                return Ok(())
            }
        }
        self.ensure_valid_position(position)?;
        let mut carry = D::ZERO;
        D::add_full(self[position], d, &mut self[position], &mut carry);
        let high = self.body_high()?;
        let mut on_position = (position + 1isize)?;
        while carry == D::ONE && on_position < high {
            D::add_full(self[on_position], D::ONE, &mut self[on_position], &mut carry);
            on_position += 1isize;
        }
        Ok(())
    }

    // mutate in place to negative
    pub fn negate(&mut self) -> Result<(), BigFixedError> {
        self.head = !self.head;
        for i in 0..self.body.len() {
            self.body[i] = !self.body[i];
        }
        self.add_digit(D::ONE, self.position)?;
        self.format()?;
        Ok(())
    }

    pub fn abs(&self) -> Result<BigFixedVec<D>, BigFixedError> {
        if self.is_neg()? {
            let mut copy = self.clone();
            copy.negate()?;
            Ok(copy)
        } else {
            Ok(self.clone())
        }
    }

    pub fn add_assign(&mut self, other: &BigFixedVec<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        let position = min(self.position, other.position);
        // one more for overflow
        let high = (max(self.body_high()?, other.body_high()?) + Index::Position(1))?;
        self.ensure_valid_range(position, high)?;
        let other_low = other.position.cast_to_position()?;
        for i in other_low.value()?..high.value()? {
            let p = Index::Position(i);
            self.add_digit_drop_overflow(other[p], p)?;
        }
        self.head = if self[(high - Index::Position(1))?] >= D::GREATESTBIT {
            D::ALLONES
        } else {
            D::ZERO
        };
        self.format()
    }

    pub fn bitand_assign(&mut self, other: &BigFixedVec<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high()?, other.body_high()?);
        self.ensure_valid_range(position, high)?;

        for i in 0..self.body.len() {
            self.body[i] &= other[(self.position + i)?];
        }

        self.head &= other.head;
        self.format()
    }

    pub fn bitor_assign(&mut self, other: &BigFixedVec<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high()?, other.body_high()?);
        self.ensure_valid_range(position, high)?;

        for i in 0..self.body.len() {
            self.body[i] |= other[(self.position + i)?];
        }

        self.head |= other.head;
        self.format()
    }

    pub fn bitxor_assign(&mut self, other: &BigFixedVec<D>) -> Result<(), BigFixedError> {
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high()?, other.body_high()?);
        self.ensure_valid_range(position, high)?;

        for i in 0..self.body.len() {
            self.body[i] ^= other[(self.position + i)?];
        }

        self.head ^= other.head;
        self.format()
    }

    pub fn mul_assign(&mut self, other: &BigFixedVec<D>) -> Result<(), BigFixedError> {
        self.fix_position()?;
        assert!(other.properly_positioned());
        // have to check for 0 anyway because of -0 issues, might as well check at the top
        if self.is_zero()? {
            return Ok(());
        }
        if other.is_zero()? {
            self.overwrite(other);
            return Ok(());
        }
        let low = (self.position + other.position)?;
        self.position = Index::Position(0);
        let mut self_len = self.body.len();
        if self.is_neg()? {
            self_len += 1;
        }
        let mut other_len = other.body.len();
        if other.is_neg()? {
            other_len += 1;
        }
        let len = 2*max(self_len, other_len);
        self.body.resize(len, self.head);
        let mut other_base;
        let mut prod_res = D::ZERO;
        let mut prod_carry = D::ZERO;
        let mut sum_res;
        let mut sum_carry = D::ZERO;
        let mut total_carry;
        let mut summ_res;
        let mut summ_carry = D::ZERO;
        let mut totall_carry;
        for i in (0..len).rev() {
            sum_res = D::ZERO;
            total_carry = D::ZERO;
            summ_res = D::ZERO;
            totall_carry = D::ZERO;
            for j in 0..=i {
                other_base = (other.position + i)?;
                D::mul_full(self.body[j], other[(other_base - j)?], &mut prod_res, &mut prod_carry);
                D::add_full(sum_res, prod_res, &mut sum_res, &mut sum_carry);
                total_carry += sum_carry;
                D::add_full(summ_res, prod_carry, &mut summ_res, &mut summ_carry);
                totall_carry += summ_carry;
            }
            self.body[i] = D::ZERO;
            self.add_digit_drop_overflow(sum_res, Index::Position(Index::<D>::castsize(i)?))?;
            self.add_digit_drop_overflow(total_carry, Index::Position(Index::<D>::castsize(i + 1)?))?;
            self.add_digit_drop_overflow(summ_res, Index::Position(Index::<D>::castsize(i + 1)?))?;
            self.add_digit_drop_overflow(totall_carry, Index::Position(Index::<D>::castsize(i + 2)?))?;
        }
        self.head = self.head ^ other.head;
        self.position = low;
        self.format()
    }

    pub fn shl_assign(&mut self, amount: &usize) -> Result<(), BigFixedError> {
        self.position = (self.position + Index::Bit(Index::<D>::castsize(*amount)?))?;
        self.format()?;
        Ok(())
    }
    
    pub fn shr_assign(&mut self, amount: &usize) -> Result<(), BigFixedError> {
        self.position = (self.position - Index::Bit(Index::<D>::castsize(*amount)?))?;
        self.format()?;
        Ok(())
    }

    // could save a few runtime steps by making a subtract_digit method but this is easier to build
    pub fn sub_assign(&mut self, other: &BigFixedVec<D>) -> Result<(), BigFixedError> {
        self.negate()?;
        self.add_assign(other)?;
        self.negate()?;
        Ok(())
    }
}

/*
op_assign_to_op!(op, op_fn_name, op_assign, op_assign_fn_name, self_type, other_type, result_type, error_type)
*/
op_assign_to_op!(D, Digit, Add, add, AddAssign, add_assign, BigFixedVec<D>, BigFixedVec<D>, BigFixedVec<D>, BigFixedError);
op_assign_to_op!(D, Digit, BitAnd, bitand, BitAndAssign, bitand_assign, BigFixedVec<D>, BigFixedVec<D>, BigFixedVec<D>, BigFixedError);
op_assign_to_op!(D, Digit, BitOr, bitor, BitOrAssign, bitor_assign, BigFixedVec<D>, BigFixedVec<D>, BigFixedVec<D>, BigFixedError);
op_assign_to_op!(D, Digit, BitXor, bitxor, BitXorAssign, bitxor_assign, BigFixedVec<D>, BigFixedVec<D>, BigFixedVec<D>, BigFixedError);
op_assign_to_op!(D, Digit, Mul, mul, MulAssign, mul_assign, BigFixedVec<D>, BigFixedVec<D>, BigFixedVec<D>, BigFixedError);
op_assign_to_op!(D, Digit, Shl, shl, ShlAssign, shl_assign, BigFixedVec<D>, usize, BigFixedVec<D>, BigFixedError);
op_assign_to_op!(D, Digit, Shr, shr, ShrAssign, shr_assign, BigFixedVec<D>, usize, BigFixedVec<D>, BigFixedError);
op_assign_to_op!(D, Digit, Sub, sub, SubAssign, sub_assign, BigFixedVec<D>, BigFixedVec<D>, BigFixedVec<D>, BigFixedError);

// additive and bitwise negation are the same thing via the geometric series trick for truncating binary expansions
unary!(D, Digit, Neg, neg, BigFixedVec<D>, negate, BigFixedVec<D>, BigFixedError);
unary!(D, Digit, Not, not, BigFixedVec<D>, negate, BigFixedVec<D>, BigFixedError);

// Rem and RemAssign depend on division

// division section --

impl<D: Digit> BigFixedVec<D> {
    pub fn combined_div(num: &mut BigFixedVec<D>, denom: &BigFixedVec<D>, end: Cutoff<D>) -> Result<BigFixedVec<D>, BigFixedError> {
        // Iteratively subtract the highest multiple of the highest shift of denom from num, storing into quotient. Num is replaced by the remainder at each step.
        // Go until num (the remainder) is small enough so that num / denom has 0s in all positions >= -to, i.e. num < denom / base^to.
        // sign stuff
        let mut quotient = BigFixedVec::from(0);
        if &*num < &quotient {
            num.negate()?;
            quotient = BigFixedVec::combined_div(num, denom, end)?;
            num.negate()?;
            quotient.negate()?;
            return Ok(quotient);
        }
        if denom < &quotient {
            quotient = BigFixedVec::combined_div(num, &(-denom)?, end)?;
            num.negate()?;
            quotient.negate()?;
            return Ok(quotient);
        }
        assert!(!denom.is_zero()?, "divide by zero");
        assert!(&*num >= &quotient && denom >= &quotient, "sign issue");

        // starting the actual division
        let denom_tail_len = denom.body.len() - 1;
        let denom_high_position = (denom.body_high()? - 1isize)?;
        let mut shifted_denom = denom.clone();
        let mut position = (num.body_high()? - 1isize)?;
        //println!("num\t{:?}", num);
        //println!("denom\t{:?}", denom);
        let mut prod = BigFixedVec::from(0);
        while !num.is_zero()? && quotient.cutoff_index(end)? <= quotient.position {
            //println!("___________________");
            //println!("division step {}", position);
            //println!("looped because {:?} is nonzero and {} <= {}", num, quotient.cutoff_index(end)?, quotient.position);
            shifted_denom.position = (position - denom_tail_len)?;
            //println!("shift d\t{:?}", shifted_denom);
            let mut quot = D::ZERO;
            //println!("num\t{:?}", num);
            //println!("position {}", position);
            D::div(num[(position + Index::Position(1))?], num[position], shifted_denom[position], &mut quot);
            prod = BigFixedVec::from(quot);
            //println!("quot {}", quot);
            //println!("prod\t{:?}", prod);
            prod *= &shifted_denom;
            //println!("scaled\t{:?}", prod);
            while &prod < num && quot < D::ALLONES {
                //println!("incrementing");
                quot += D::ONE;
                //println!("prod\t{:?}", prod);
                prod += &shifted_denom;
                //println!("prod\t{:?}", prod);
            }
            while &prod > num && quot > D::ZERO {
                //println!("decrementing");
                quot -= D::ONE;
                //println!("prod\t{:?}", prod);
                prod -= &shifted_denom;
                //println!("prod\t{:?}", prod);
            }
            //println!("fixed quot {}", quot);
            //println!("fixed prod {:?}", prod);
            quotient[(position - denom_high_position)?] = quot;
            //println!("quotient\t{:?}", quotient);
            //println!("pre subtraction {:?} - {:?}", num, prod);
            *num -= &prod;
            //println!("rem\t{:?}", num);
            position -= 1isize;
            //println!("___________________");
        }
        //println!("wrapup cutoff");
        quotient.format()?;
        // use prod to hold the difference between quotient and its cutoff
        prod.overwrite(&quotient);
        quotient.cutoff(end)?;
        prod -= &quotient;
        prod *= denom;
        *num += &prod;
        //println!("done\n");
        Ok(quotient)
    }

    pub fn to_digits(&self, base: &BigFixedVec<D>) -> Result<(Vec<BigFixedVec<D>>, isize), BigFixedError> {
        let mut shifting = self.abs()?.clone();
        let mut neg_count: isize = 0;
        //println!("to digits\t{:?}", shifting);
        while shifting.position < 0isize {
            shifting *= base;
            neg_count += 1;
        }
        //println!("to digits starting with {:?} which has been shifted {}", shifting, neg_count);
        let mut digits = vec![];
        while !shifting.is_zero()? {
            let quot = BigFixedVec::combined_div(&mut shifting, base, Cutoff::INTEGER)?;
            digits.push(shifting.clone());
            shifting = quot;
        }
        //println!("digits are {:?} pt {}", digits, Index::Position(neg_count));
        Ok((digits, neg_count))
    }

    pub fn to_digits_10(&self) -> Result<(Vec<i32>, isize), BigFixedError> {
        let (big_digits, point) = self.to_digits(&BigFixedVec::from(10))?;
        let digits: Vec<i32> = big_digits.iter().map(|x| i32::from(x)).collect();
        Ok((digits, point))
    }
}

// -- end division section

impl<D: Digit> PartialOrd for BigFixedVec<D> {
    fn partial_cmp(&self, other: &BigFixedVec<D>) -> Option<Ordering> {
        let mut step_result = self.head.partial_cmp(&other.head)?;
        match step_result {
            Ordering::Equal => {
                for i in (
                    min(self.position.cast_to_position().ok()?, other.position.cast_to_position().ok()?).value().ok()?..
                    (max(self.body_high().ok()?.cast_to_position().ok()?, other.body_high().ok()?.cast_to_position().ok()?)).value().ok()?
                ).rev() {
                    step_result = self[i].partial_cmp(&other[i])?;
                    match step_result {
                        Ordering::Equal => continue,
                        x => return Some(x)
                    }
                }
                Some(Ordering::Equal)
            },
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Greater => Some(Ordering::Less)
        }
    }
}
