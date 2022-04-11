use crate::{digit::*, BigFixed};

use std::{ops::*, cmp::*, option::*};

impl BigFixed {
    // Add digit into position and handle carries
    pub fn add_digit(&mut self, d: Digit, position: isize) {
        let mut res;
        let mut carry;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.body_high();
        let mut on_position = position + 1;
        while carry == 1 && on_position < high {
            add!(self[on_position], 1, res, carry);
            self[on_position] = res;
            on_position += 1;
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

    pub fn add_digit_drop_overflow(&mut self, d: Digit, position: isize) {
        if position >= self.body_high() {
            // already overflows
            return;
        }
        let mut res;
        let mut carry;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.body_high();
        let mut on_position = position + 1;
        while carry == 1 && on_position < high {
            add!(self[on_position], 1, res, carry);
            self[on_position] = res;
            on_position += 1;
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

/*
    This macro is for extending operations to various combinations of owned/referenced objects. Everything can work using only references but sometimes
    the code can be made to look cleaner by omitting explicit references if possible. Assume we have constructed op_assign on references, i.e.
     -- impl $op_assign<&$other_type> for $self_type {
     --     fn $op_assign_fn_name(&mut self, other: &$other_type) {...}
     -- }

     That is, if `a` is a self_type and `b` is an other_type then `a.op_assign_fn_name(&b)` makes sense. This macro extends the functionality to
     -- a.op_assign_fn_name(b)
     -- (&a).op_fn_name(&b)
     -- (&a).op_fn_name(b)
     -- a.op_fn_name(&b)
     -- a.op_fn_name(b)
    by fiddling with reference creation.
*/

macro_rules! op_extension {
    ($op: ident, $op_assign: ident, $op_fn_name: ident, $op_assign_fn_name: ident, $self_type: ty, $other_type: ty) => {
        impl $op_assign<$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: $other_type) {
                self.$op_assign_fn_name(&other);
            }
        }
        impl $op<&$other_type> for &$self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: &$other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
        impl $op<$other_type> for &$self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: $other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
        impl $op<&$other_type> for $self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: &$other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
        impl $op<$other_type> for $self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: $other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
    };
    // unary operation version
    ($op: path, $op_fn_name: ident, $self_type: ty) => {
        impl $op for $self_type {
            type Output = $self_type;
            fn $op_fn_name(self) -> $self_type {
                (&self).$op_fn_name()
            }
        }
    };
}

impl AddAssign<&BigFixed> for BigFixed {
    fn add_assign(&mut self, other: &BigFixed) {
        // align self valid range
        let position = min(self.position, other.position);
        let high = max(self.body_high(), other.body_high());
        // one extra in case of overflow
        self.ensure_valid_range(position, high + 1);
        
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

// DerefMut is only for smart pointers and that's not what this is

// Div is complicated -- deal with it later

// DivAssign ^

// Drop ^^

// Fn* -- not relevant

impl Index<isize> for BigFixed {
    type Output = Digit;
    fn index(&self, position: isize) -> &Digit {
        let shifted = position - self.position;
        if shifted >= self.body.len() as isize {
            &self.head
        } else if shifted >= 0 {
            &self.body[shifted as usize]
        } else {
            &0
        }
    }
}

impl IndexMut<isize> for BigFixed {
    fn index_mut(&mut self, position: isize) -> &mut Digit {
        self.ensure_valid_position(position);
        self.body.index_mut((position - self.position) as usize)
    }
}

impl MulAssign<&BigFixed> for BigFixed {
    fn mul_assign(&mut self, other: &BigFixed) {
        println!("multiplying {} x {}", self, other);
        let mut result = BigFixed::from(0 as Digit);
        let low = self.position + other.position;
        result.position = low;
        let mut self_len = self.body.len() as isize;
        // numbers like -1 (-2^k) have empty bodies
        if self.is_neg() {
            self_len += 1;
        }
        let mut other_len = other.body.len() as isize;
        if other.is_neg() {
            other_len += 1;
        }
        let len = max(self_len, other_len);
        let high = low + 2*len;
        result.ensure_valid_range(low, high);
        let mut sum_position;
        let mut prod_res: Digit;
        let mut prod_carry: Digit;
        for i in self.position..high as isize {
            for j in other.position..max(other.position, high - i) as isize {
                sum_position = i + j;
                mul!(self[i], other[j], prod_res, prod_carry);
                result.add_digit_drop_overflow(prod_res, sum_position);
                result.add_digit_drop_overflow(prod_carry, sum_position + 1);
            }
        }
        result.head = self.head ^ other.head;
        result.format();
        self.overwrite(result);
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

// RangeBounds does not apply to individual BigFixeds

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
            self.ensure_valid_position(self.position - 1);
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
        *self += &-other;
    }
}

op_extension!(Add, AddAssign, add, add_assign, BigFixed, BigFixed);
op_extension!(BitAnd, BitAndAssign, bitand, bitand_assign, BigFixed, BigFixed);
op_extension!(BitOr, BitOrAssign, bitor, bitor_assign, BigFixed, BigFixed);
op_extension!(BitXor, BitXorAssign, bitxor, bitxor_assign, BigFixed, BigFixed);
op_extension!(Mul, MulAssign, mul, mul_assign, BigFixed, BigFixed);
op_extension!(Neg, neg, BigFixed);
op_extension!(Not, not, BigFixed);
op_extension!(Shl, ShlAssign, shl, shl_assign, BigFixed, usize);
op_extension!(Shr, ShrAssign, shr, shr_assign, BigFixed, usize);
op_extension!(Sub, SubAssign, sub, sub_assign, BigFixed, BigFixed);

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
                for i in (min(self.position, other.position)..max(self.body_high(), other.body_high())).rev() {
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
