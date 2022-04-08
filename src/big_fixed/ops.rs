use crate::{digit::*, Tail, BigFixed};

use std::{ops::*, cmp::{max, min}};

use num::{integer::{lcm}};

impl BigFixed {
    // Add digit into position and handle carries
    pub fn add_digit(&mut self, d: Digit, position: isize) {
        let mut res;
        let mut carry = 0;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.body_high();
        let mut on_position = position + 1;
        while carry == 1 && on_position < high {
            carry = 0;
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
        self.ensure_valid_range(position, high);
        
        // add heads
        self.head ^= other.head;
        
        // add tails
        let tail_len = lcm(self.tail.len(), other.tail.len());
        let tail_low = self.position - tail_len as isize;
        self.tail.resize(tail_len);
        let mut res;
        let mut carry = 0;
        let mut previous_carry;
        for i in 0..tail_len {
            previous_carry = carry;
            carry = 0;
            add!(self.tail[i], previous_carry, res, carry);
            self.tail[i] = res;
            add!(self.tail[i], other[tail_low + i as isize], res, carry);
            self.tail[i] = res;
        }
        if carry == 1 {
            self.add_digit(1, self.position);
            for carry_in_tail in 0..tail_len {
                previous_carry = carry;
                carry = 0;
                add!(self.tail[carry_in_tail], previous_carry, res, carry);
                self.tail[carry_in_tail] = res;
                if carry == 0 {break};
            }
        }
        debug_assert_eq!(carry, 0, "double carry");
        
        // add bodies
        for in_body in 0..self.body.len() {
            add!(self.body[in_body], carry, res);
            self.body[in_body] = res;
            carry = 0;
            add!(self.body[in_body], other[self.position + in_body as isize], res, carry);
            self.body[in_body] = res;
        }
        if carry == 1 {
            if self.is_neg() {
                self.head = 0;
            } else {
                self.add_digit(1, self.body_high());
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

        let tail_len = lcm(self.tail.len(), other.tail.len());
        let tail_low = self.position - tail_len as isize;
        self.tail.resize(tail_len);
        for i in 0..tail_len {
            self.tail[i] &= other[tail_low + i as isize];
        }

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

        let tail_len = lcm(self.tail.len(), other.tail.len());
        let tail_low = self.position - tail_len as isize;
        self.tail.resize(tail_len);
        for i in 0..tail_len {
            self.tail[i] |= other[tail_low + i as isize];
        }

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

        let tail_len = lcm(self.tail.len(), other.tail.len());
        let tail_low = self.position - tail_len as isize;
        self.tail.resize(tail_len);
        for i in 0..tail_len {
            self.tail[i] ^= other[tail_low + i as isize];
        }

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
            &self.tail[shifted]
        }
    }
}

impl IndexMut<isize> for BigFixed {
    fn index_mut(&mut self, position: isize) -> &mut Digit {
        self.ensure_valid_position(position);
        self.body.index_mut((position - self.position) as usize)
    }
}

/*
Mul

The multiplication operator *.
MulAssign

The multiplication assignment operator *=.
*/

impl Neg for &BigFixed {
    type Output = BigFixed;
    fn neg(self) -> BigFixed {
        !self
    }
}

impl Not for &BigFixed {
    type Output = BigFixed;
    fn not(self) -> BigFixed {
        BigFixed::construct(
            !self.head,
            self.body.iter().map(|x| !x).collect(),
            Tail::from(self.tail.data.iter().map(|x| !x).collect::<Vec<Digit>>()),
            self.position
        )
    }
}

// RangeBounds under construction... May end up being something different

// Rem and RemAssign are trivial since division is perfect but we can implement them anyway once division is done

impl ShlAssign<&usize> for BigFixed {
    fn shl_assign(&mut self, amount: &usize) {
        let places = amount / DIGITBITS;
        let subshift = amount % DIGITBITS;
        self.position += places as isize;
        if subshift > 0 {
            let opsubshift = DIGITBITS - subshift;
            let keepmask = ALLONES >> opsubshift;
            let carrymask = !keepmask;
            let high = self.body_high();
            self.ensure_valid_position(high);
            for i in (1..self.body.len()).rev() {
                self.body[i] =
                    ((self.body[i] & keepmask) << subshift)
                    | ((self.body[i-1] & carrymask) >> opsubshift);
            }
            self.body[0] = ((self.body[0] & keepmask) << subshift) | ((self.tail[self.tail.len() - 1] & carrymask) >> opsubshift);
            let old_high = self.tail[-1];
            for i in (1..self.tail.len() as isize).rev() {
                self.tail[i] =
                    ((self.tail[i] & keepmask) << subshift)
                    | ((self.tail[i-1] & carrymask) >> opsubshift);
            }
            self.tail[0] = ((self.tail[0] & keepmask) << subshift) | ((old_high & carrymask) >> opsubshift);
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
            let old_low = self.tail[0];
            for i in 1..self.tail.len() as isize {
                self.tail[i-1] =
                    ((self.tail[i-1] & keepmask) >> subshift)
                    | ((self.tail[i] & carrymask) << opsubshift);
            }
            self.tail[-1] = ((self.tail[-1] & keepmask) >> subshift) | ((old_low & carrymask) << opsubshift);
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
op_extension!(Neg, neg, BigFixed);
op_extension!(Not, not, BigFixed);
op_extension!(Shl, ShlAssign, shl, shl_assign, BigFixed, usize);
op_extension!(Shr, ShrAssign, shr, shr_assign, BigFixed, usize);
op_extension!(Sub, SubAssign, sub, sub_assign, BigFixed, BigFixed);
