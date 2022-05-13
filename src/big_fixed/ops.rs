use crate::{digit::*, Index, BigFixed, BigFixedError, macros::*};

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

impl BigFixed {
    // Add digit into position and handle carries
    pub fn add_digit(&mut self, d: Digit, position: Index) -> Result<(), BigFixedError> {
        assert!(self.properly_positioned());
        if let Index::Bit(_) = position {
            let diff = position.bit_position_excess();
            if diff == 0 {
                return self.add_digit(d, position.cast_to_position());
            }
            let as_position = position.cast_to_position();
            self.add_digit(d >> (DIGITBITS as isize - diff), (as_position + Index::Position(1))?)?;
            self.add_digit(d << diff, as_position)?;
            return Ok(())
        }
        self.ensure_valid_position(position)?;
        let mut res;
        let mut carry;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.body_high()?;
        let mut on_position = (position + 1isize)?;
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
        };
        Ok(())
    }

    // add_digit but leaves (positionally entire) head unchanged
    pub fn add_digit_drop_overflow(&mut self, d: Digit, position: Index) -> Result<(), BigFixedError> {
        assert!(self.properly_positioned());
        if position >= self.body_high()? {
            // already overflows
            return Ok(());
        }
        if let Index::Bit(_b) = position {
            if let Index::Bit(_) = position {
                let diff = position.bit_position_excess();
                let as_position = position.cast_to_position();
                self.add_digit_drop_overflow(d >> (DIGITBITS as isize - diff), (as_position + Index::Position(1))?)?;
                self.add_digit_drop_overflow(d << diff, as_position)?;
                return Ok(())
            }
        }
        self.ensure_valid_position(position)?;
        let mut res;
        let mut carry;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.body_high()?;
        let mut on_position = (position + 1isize)?;
        while carry == 1 && on_position < high {
            add!(self[on_position], 1, res, carry);
            self[on_position] = res;
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
        self.add_digit(1, self.position)?;
        self.format()?;
        Ok(())
    }

    pub fn abs(&self) -> Result<BigFixed, BigFixedError> {
        if self.is_neg() {
            let mut copy = self.clone();
            copy.negate()?;
            Ok(copy)
        } else {
            Ok(self.clone())
        }
    }

    fn add_assign(&mut self, other: &BigFixed) -> Result<(), BigFixedError> {
        self.fix_position()?;
        let position = min(self.position, other.position);
        // one more for overflow
        let high = (max(self.body_high()?, other.body_high()?) + Index::Position(1))?;
        self.ensure_valid_range(position, high)?;
        let other_low = other.position.cast_to_position();
        for i in other_low.value()..high.value() {
            let p = Index::Position(i);
            self.add_digit_drop_overflow(other[p], p)?;
        }
        self.head = if self[(high - Index::Position(1))?] >= GREATESTBIT {
            ALLONES
        } else {
            0
        };
        self.format()
    }

    pub fn bitand_assign(&mut self, other: &BigFixed) -> Result<(), BigFixedError> {
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

    pub fn bitor_assign(&mut self, other: &BigFixed) -> Result<(), BigFixedError> {
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

    pub fn bitxor_assign(&mut self, other: &BigFixed) -> Result<(), BigFixedError> {
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

    pub fn mul_assign(&mut self, other: &BigFixed) -> Result<(), BigFixedError> {
        self.fix_position()?;
        assert!(other.properly_positioned());
        // have to check for 0 anyway because of -0 issues, might as well check at the top
        if self.is_zero() {
            return Ok(());
        }
        if other.is_zero() {
            self.overwrite(other);
            return Ok(());
        }
        let low = (self.position + other.position)?;
        self.position = Index::Position(0);
        let mut self_len = self.body.len();
        if self.is_neg() {
            self_len += 1;
        }
        let mut other_len = other.body.len();
        if other.is_neg() {
            other_len += 1;
        }
        let len = 2*max(self_len, other_len);
        self.body.resize(len, self.head);
        let mut other_base;
        let mut prod_res;
        let mut prod_carry;
        let mut sum_res;
        let mut sum_carry;
        let mut total_carry;
        let mut summ_res;
        let mut summ_carry;
        let mut totall_carry;
        for i in (0..len).rev() {
            sum_res = 0;
            total_carry = 0;
            summ_res = 0;
            totall_carry = 0;
            for j in 0..=i {
                other_base = (other.position + i)?;
                mul!(self.body[j], other[(other_base - j)?], prod_res, prod_carry);
                add!(sum_res, prod_res, sum_res, sum_carry);
                total_carry += sum_carry;
                add!(summ_res, prod_carry, summ_res, summ_carry);
                totall_carry += summ_carry;
            }
            self.body[i] = 0;
            self.add_digit_drop_overflow(sum_res, Index::Position(Index::castsize(i)?))?;
            self.add_digit_drop_overflow(total_carry, Index::Position(Index::castsize(i + 1)?))?;
            self.add_digit_drop_overflow(summ_res, Index::Position(Index::castsize(i + 1)?))?;
            self.add_digit_drop_overflow(totall_carry, Index::Position(Index::castsize(i + 2)?))?;
        }
        self.head = self.head ^ other.head;
        self.position = low;
        self.format()
    }

    pub fn shl_assign(&mut self, amount: &usize) -> Result<(), BigFixedError> {
        self.position = (self.position + Index::Bit(Index::castsize(*amount)?))?;
        self.fix_position()?;
        Ok(())
    }
    
    pub fn shr_assign(&mut self, amount: &usize) -> Result<(), BigFixedError> {
        self.position = (self.position - Index::Bit(Index::castsize(*amount)?))?;
        self.fix_position()?;
        Ok(())
    }

    // could save a few runtime steps by making a subtract_digit method but this is easier to build
    pub fn sub_assign(&mut self, other: &BigFixed) -> Result<(), BigFixedError> {
        self.negate()?;
        self.add_assign(other)?;
        self.negate()?;
        Ok(())
    }
}

/*
op_assign_to_op!(op, op_fn_name, op_assign, op_assign_fn_name, self_type, other_type, result_type, error_type)
*/
op_assign_to_op!(Add, add, AddAssign, add_assign, BigFixed, BigFixed, BigFixed, BigFixedError);
op_assign_to_op!(BitAnd, bitand, BitAndAssign, bitand_assign, BigFixed, BigFixed, BigFixed, BigFixedError);
op_assign_to_op!(BitOr, bitor, BitOrAssign, bitor_assign, BigFixed, BigFixed, BigFixed, BigFixedError);
op_assign_to_op!(BitXor, bitxor, BitXorAssign, bitxor_assign, BigFixed, BigFixed, BigFixed, BigFixedError);
op_assign_to_op!(Mul, mul, MulAssign, mul_assign, BigFixed, BigFixed, BigFixed, BigFixedError);
op_assign_to_op!(Shl, shl, ShlAssign, shl_assign, BigFixed, usize, BigFixed, BigFixedError);
op_assign_to_op!(Shr, shr, ShrAssign, shr_assign, BigFixed, usize, BigFixed, BigFixedError);
op_assign_to_op!(Sub, sub, SubAssign, sub_assign, BigFixed, BigFixed, BigFixed, BigFixedError);

// additive and bitwise negation are the same thing via the geometric series trick for truncating binary expansions
unary!(Neg, neg, BigFixed, negate, BigFixed, BigFixedError);
unary!(Not, not, BigFixed, negate, BigFixed, BigFixedError);

// Rem and RemAssign depend on division

impl BigFixed {
    pub fn combined_div(num: &mut BigFixed, denom: &BigFixed, to: usize) -> Result<BigFixed, BigFixedError> {
        // Iteratively subtract the highest multiple of the highest shift of denom from num, storing into quotient. Num is replaced by the remainder at each step.
        // Go until num (the remainder) is small enough so that num / denom has 0s in all positions >= -to, i.e. num < denom / base^to.
        // sign stuff
        let mut quotient = BigFixed::from(0);
        if &*num < &quotient {
            num.negate()?;
            quotient = BigFixed::combined_div(num, denom, to)?;
            num.negate()?;
            quotient.negate()?;
            return Ok(quotient);
        }
        if denom < &quotient {
            quotient = BigFixed::combined_div(num, &(-denom)?, to)?;
            num.negate()?;
            quotient.negate()?;
            return Ok(quotient);
        }
        assert!(!denom.is_zero(), "divide by zero");
        assert!(&*num >= &quotient && denom >= &quotient, "sign issue");

        // starting the actual division
        // the cutoff is denom * base^-to
        let mut cutoff = BigFixed::from(1).shift((-Index::Position(Index::castsize(to)?))?)?;
        cutoff *= denom;
        let denom_tail_len = denom.body.len() - 1;
        let denom_high_position = (denom.body_high()? - 1isize)?;
        let mut shifted_denom = denom.clone();
        let mut position = (num.body_high()? - 1isize)?;
        //println!("num\t{:?}", num);
        while !num.is_zero() && &*num >= &cutoff {
            //println!("___________________");
            shifted_denom.position = (position - denom_tail_len)?;
            //println!("shift d\t{:?}", shifted_denom);
            let mut quot;
            //println!("num\t{:?}", num);
            //println!("position {}", position);
            div!(num[(position + 1isize)?], num[position], shifted_denom[position], quot);
            let mut prod = BigFixed::from(quot);
            //println!("quot {}", quot);
            //println!("prod\t{:?}", prod);
            prod *= &shifted_denom;
            //println!("scaled\t{:?}", prod);
            while &prod < num && quot < ALLONES {
                //println!("incrementing");
                quot += 1;
                //println!("prod\t{:?}", prod);
                prod += &shifted_denom;
                //println!("prod\t{:?}", prod);
            }
            while &prod > num && quot > 0 {
                //println!("decrementing");
                quot -= 1;
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
        //println!("done\n");
        quotient.format()?;
        Ok(quotient)
    }

    pub fn to_digits(&self, base: &BigFixed) -> Result<(Vec<BigFixed>, Index), BigFixedError> {
        let mut shifting = self.abs()?.clone();
        let mut neg_count: isize = 0;
        //println!("to digits\t{:?}", shifting);
        while shifting.position < 0isize {
            shifting *= base;
            neg_count += 1;
        }
        //println!("to digits starting with {:?} which has been shifted {}", shifting, neg_count);
        let mut digits = vec![];
        while !shifting.is_zero() {
            let quot = BigFixed::combined_div(&mut shifting, base, 0)?;
            digits.push(shifting.clone());
            shifting = quot;
        }
        //println!("digits are {:?} pt {}", digits, Index::Position(neg_count));
        Ok((digits, Index::Position(neg_count)))
    }

    pub fn to_digits_10(&self) -> Result<(Vec<i32>, Index), BigFixedError> {
        let (big_digits, point) = self.to_digits(&BigFixed::from(10))?;
        let digits: Vec<i32> = big_digits.iter().map(|x| i32::from(x)).collect();
        Ok((digits, point))
    }
}

impl PartialOrd for BigFixed {
    fn partial_cmp(&self, other: &BigFixed) -> Option<Ordering> {
        let mut step_result = self.head.cmp(&other.head);
        match step_result {
            Ordering::Equal => {
                for i in (
                    min(self.position.cast_to_position(), other.position.cast_to_position()).value()..
                    (max(self.body_high().ok()?.cast_to_position(), other.body_high().ok()?.cast_to_position())).value()
                ).rev() {
                    step_result = self[i].cmp(&other[i]);
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
