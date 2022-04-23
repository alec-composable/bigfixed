use crate::{digit::*, Index as Indx, Cutoff, CutsOff, BigFixed, macros::*};

use std::{ops::*, cmp::*, option::*};

impl BigFixed {
    // Add digit into position and handle carries
    pub fn add_digit(&mut self, d: Digit, position: Indx) {
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

    pub fn add_digit_drop_overflow(&mut self, d: Digit, position: Indx) {
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

    // mutate in place to negative
    pub fn negate(&mut self) {
        self.head = !self.head;
        for i in 0..self.body.len() {
            self.body[i] = !self.body[i];
        }
        self.add_digit(1, self.position);
        self.format();
    }
}

impl AddAssign<&BigFixed> for BigFixed {
    fn add_assign(&mut self, other: &BigFixed) {
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high(), other.body_high());
        // one extra in case of overflow
        self.ensure_valid_range(position, high + 1isize);
        
        // add heads
        self.head ^= other.head;

        let mut res;
        let mut carry = 0;
        let mut prev_carry;
        // add bodies
        for in_body in 0..self.body.len() {
            prev_carry = carry;
            add!(self.body[in_body], prev_carry, res, carry);
            self.body[in_body] = res;
            // overloading prev_carry
            add!(self.body[in_body], other[self.position + in_body as isize], res, prev_carry);
            self.body[in_body] = res;
            carry += prev_carry;
        }
        if carry == 1 {
            if self.is_neg() {
                self.head = 0;
            } else {
                // overflow
                self[high] = 1;
            }
        }
        
        self.format();
    }
}

impl BitAndAssign<&BigFixed> for BigFixed {
    fn bitand_assign(&mut self, other: &BigFixed) {
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high(), other.body_high());
        self.ensure_valid_range(position, high);

        for i in 0..self.body.len() {
            self.body[i] &= other[self.position + i as isize];
        }

        self.head &= other.head;
    }
}

impl BitOrAssign<&BigFixed> for BigFixed {
    fn bitor_assign(&mut self, other: &BigFixed) {
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high(), other.body_high());
        self.ensure_valid_range(position, high);

        for i in 0..self.body.len() {
            self.body[i] |= other[self.position + i as isize];
        }

        self.head |= other.head;
    }
}

impl BitXorAssign<&BigFixed> for BigFixed {
    fn bitxor_assign(&mut self, other: &BigFixed) {
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high(), other.body_high());
        self.ensure_valid_range(position, high);

        for i in 0..self.body.len() {
            self.body[i] ^= other[self.position + i as isize];
        }

        self.head ^= other.head;
    }
}

impl BigFixed {
    pub fn combined_div(num: &mut BigFixed, denom: &BigFixed, to: usize) -> BigFixed {
        // Iteratively subtract the highest multiple of the highest shift of denom from num, storing into quotient. Num is replaced by the remainder at each step.
        // Go until num (the remainder) is small enough so that num / denom has 0s in all positions >= -to, i.e. num < denom / base^to.
        // sign stuff
        let mut quotient = BigFixed::from(0);
        if &*num < &quotient {
            num.negate();
            quotient = BigFixed::combined_div(num, denom, to);
            num.negate();
            quotient.negate();
            return quotient;
        }
        if denom < &quotient {
            quotient = BigFixed::combined_div(num, &-denom, to);
            num.negate();
            quotient.negate();
            return quotient;
        }
        assert!(!denom.is_zero(), "divide by zero");
        assert!(&*num >= &quotient && denom >= &quotient, "sign issue");

        // the cutoff is denom * base^-to
        let mut cutoff = BigFixed::from(1isize).shift(-Indx::cast(to));
        cutoff *= denom;
        // if the cutoff is bigger then division gives 0
        if &*num < &cutoff {
            return quotient;
        }

        // starting the actual division
        let denom_tail_len = denom.body.len() - 1;
        let denom_high_position = denom.body_high() - 1isize;
        let mut shifted_denom = denom.clone();
        let mut position = num.body_high() - 1isize;
        while !num.is_zero() && &*num >= &cutoff {
            shifted_denom.position = position - denom_tail_len;
            let mut quot;
            div!(num[position], num[position - 1isize], shifted_denom[position], shifted_denom[position - 1isize], quot);
            let mut prod = BigFixed::from(quot);
            prod *= &shifted_denom;
            while &prod < num {
                quot += 1;
                prod += &shifted_denom;
            }
            while &prod > num {
                quot -= 1;
                prod -= &shifted_denom;
            }
            quotient[position - denom_high_position] = quot;
            *num -= &prod;
            position -= 1isize;
        }
        quotient.format();
        quotient
    }

    pub fn to_digits(&self, base: &BigFixed) -> (Vec<BigFixed>, Indx) {
        println!("self\t{}", self);
        println!("base\t{}", base);
        let mut shifting = self.clone();
        let mut neg_count: isize = 0;
        while shifting.position < Indx::ZERO {
            shifting *= base;
            neg_count += 1;
        }
        if neg_count > 0 {
            println!("neg count {}", neg_count);
            println!("shifted\t{}", shifting);
        }
        let mut digits = vec![];
        println!("looping");
        println!("");
        while !shifting.is_zero() {
            //println!("shifting\t{}", shifting);
            let quot = BigFixed::combined_div(&mut shifting, base, 0);
            //println!("quot\t{}", quot);
            //println!("rem\t{}", shifting);
            digits.push(shifting.clone());
            shifting = quot;
            //println!("");
        }
        (digits, neg_count.into())
    }
}

impl Index<Indx> for BigFixed {
    type Output = Digit;
    fn index(&self, position: Indx) -> &Digit {
        let shifted = position - self.position;
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
        &self[Indx::from(position)]
    }
}

impl IndexMut<Indx> for BigFixed {
    fn index_mut(&mut self, position: Indx) -> &mut Digit {
        self.ensure_valid_position(position);
        self.body.index_mut(usize::from(position - self.position))
    }
}

impl IndexMut<isize> for BigFixed {
    fn index_mut(&mut self, position: isize) -> &mut Digit {
        self.index_mut(Indx::from(position))
    }
}

impl MulAssign<&BigFixed> for BigFixed {
    fn mul_assign(&mut self, other: &BigFixed) {
        // have to check for 0 anyway because of -0 issues, might as well check at the top
        if self.is_zero() {
            return;
        }
        if other.is_zero() {
            self.head = 0;
            self.cutoff(Cutoff::from((self.body_high(), Indx::ZERO)));
            return;
        }
        let low = self.position + other.position;
        self.position = Indx::ZERO;
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
                other_base = other.position + i as isize;
                mul!(self.body[j], other[other_base - j as isize], prod_res, prod_carry);
                add!(sum_res, prod_res, sum_res, sum_carry);
                total_carry += sum_carry;
                add!(summ_res, prod_carry, summ_res, summ_carry);
                totall_carry += summ_carry;
            }
            self.body[i] = 0;
            self.add_digit_drop_overflow(sum_res, Indx::from(i));
            self.add_digit_drop_overflow(total_carry, Indx::from(i + 1));
            self.add_digit_drop_overflow(summ_res, Indx::from(i + 1));
            self.add_digit_drop_overflow(totall_carry, Indx::from(i + 2));
        }
        self.head = self.head ^ other.head;
        self.position = low;
        self.format();
    }
}

impl Neg for &BigFixed {
    type Output = BigFixed;
    fn neg(self) -> BigFixed {
        !self
    }
}

impl Not for &BigFixed {
    type Output = BigFixed;
    fn not(self) -> BigFixed {
        let mut res = self.clone();
        res.negate();
        res
    }
}

// Rem and RemAssign depend on division

impl ShlAssign<&usize> for BigFixed {
    fn shl_assign(&mut self, amount: &usize) {
        let places = amount / DIGITBITS;
        let subshift = amount % DIGITBITS;
        self.position += places as isize;
        if subshift > 0 {
            let keepmask = ALLONES >> subshift;
            let carrymask = !keepmask;
            let opsubshift = DIGITBITS - subshift;
            let high = self.body_high();
            self.ensure_valid_position(high);
            for i in (1..self.body.len()).rev() {
                self.body[i] =
                    ((self.body[i] & keepmask) << subshift)
                    | ((self.body[i-1] & carrymask) >> opsubshift);
            }
            self.body[0] = (self.body[0] & keepmask) << subshift;
            self.format();
        }
    }
}

impl ShrAssign<&usize> for BigFixed {
    fn shr_assign(&mut self, amount: &usize) {
        let places = amount / DIGITBITS;
        let subshift = amount % DIGITBITS;
        self.position -= places as isize;
        if subshift > 0 {
            let opsubshift = DIGITBITS - subshift;
            let carrymask = ALLONES >> opsubshift;
            let keepmask = !carrymask;
            self.ensure_valid_position(self.position - 1isize);
            for i in 1..self.body.len() {
                self.body[i-1] =
                    ((self.body[i-1] & keepmask) >> subshift)
                    | ((self.body[i] & carrymask) << opsubshift);
            }
            let high = self.body.len() - 1;
            self.body[high] = ((self.body[high] & keepmask) >> subshift) | ((self.head & carrymask) << opsubshift);
            self.format();
        }
    }
}

impl SubAssign<&BigFixed> for BigFixed {
    fn sub_assign(&mut self, other: &BigFixed) {
        // should probably rewrite this to not allocate unnecessarily
        *self += &-other;
    }
}

op_assign_to_op!(Add, add, AddAssign, add_assign, BigFixed, BigFixed);
op_assign_to_op!(BitAnd, bitand, BitAndAssign, bitand_assign, BigFixed, BigFixed);
op_assign_to_op!(BitOr, bitor, BitOrAssign, bitor_assign, BigFixed, BigFixed);
op_assign_to_op!(BitXor, bitxor, BitXorAssign, bitxor_assign, BigFixed, BigFixed);
op_assign_to_op!(Mul, mul, MulAssign, mul_assign, BigFixed, BigFixed);
unary!(Neg, neg, BigFixed);
unary!(Not, not, BigFixed);
op_assign_to_op!(Shl, shl, ShlAssign, shl_assign, BigFixed, usize);
op_assign_to_op!(Shr, shr, ShrAssign, shr_assign, BigFixed, usize);
op_assign_to_op!(Sub, sub, SubAssign, sub_assign, BigFixed, BigFixed);

impl PartialEq for BigFixed {
    fn eq(&self, other: &BigFixed) -> bool {
        self.position == other.position &&
        self.head == other.head &&
        self.body.len() == other.body.len() &&
        self.body == other.body
    }
}

impl Eq for &BigFixed {}

impl PartialOrd for BigFixed {
    fn partial_cmp(&self, other: &BigFixed) -> Option<Ordering> {
        let mut step_result = self.head.cmp(&other.head);
        match step_result {
            Ordering::Equal => {
                for i in (
                    min(self.position, other.position).to(
                        &max(self.body_high(), other.body_high())
                    )
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
