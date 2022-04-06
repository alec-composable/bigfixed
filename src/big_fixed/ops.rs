use crate::{digit::*, BigFixed};

use std::{ops,/* cmp::{max, min}*/};

impl BigFixed {
    // Add digit into position and handle carries
    pub fn add_digit(&mut self, d: Digit, position: isize) {
        /*let mut res;
        let mut carry;
        add!(self[position], d, res, carry);
        self[position] = res;
        let high = self.position_high();
        position += 1;
        while carry == 1 && position < high {
            add!(self.get(position), 1, res, carry);
            self.set(res, position);
            position += 1;
        }*/
        println!("adding {} pos {} to {}", d, position, self);
    }
}
/*
impl ops::Add for &BigFixed {
    type Output = BigFixed;
    fn add(self, other: &BigFixed) -> BigFixed {
        let mut res = self.clone();
        res += other;
        res
    }
}

impl ops::AddAssign<&BigFixed> for BigFixed {
    fn add_assign(&mut self, other: &BigFixed) {
        let low = min(self.position, other.position);
        // one extra in case of carry
        let high = max(self.position_high(), other.position_high()) + 1;
        self.expand_range(low, high);
        for position in low..high {
            self.add_digit(other.get(position), position);
        }
    }
}

impl ops::BitAnd for &BigFixed {
    type Output = BigFixed;
    fn bitand(self, other: &BigFixed) -> BigFixed {
        let mut res = self.clone();
        res &= other;
        res
    }
}

impl ops::BitAndAssign<&BigFixed> for BigFixed {
    fn bitand_assign(&mut self, other: &BigFixed) {
        let low = min(self.position, other.position);
        let high = max(self.position_high(), other.position_high());
        self.expand_range(low, high);
        for position in low..high {
            self.set(self.get(position) & other.get(position), position);
        }
    }
}

impl ops::BitOr for &BigFixed {
    type Output = BigFixed;
    fn bitor(self, other: &BigFixed) -> BigFixed {
        let mut res = self.clone();
        res |= other;
        res
    }
}

impl ops::BitOrAssign<&BigFixed> for BigFixed {
    fn bitor_assign(&mut self, other: &BigFixed) {
        let low = min(self.position, other.position);
        let high = max(self.position_high(), other.position_high());
        self.expand_range(low, high);
        for position in low..high {
            self.set(self.get(position) | other.get(position), position);
        }
    }
}

impl ops::BitXor for &BigFixed {
    type Output = BigFixed;
    fn bitxor(self, other: &BigFixed) -> BigFixed {
        let mut res = self.clone();
        res ^= other;
        res
    }
}

impl ops::BitXorAssign<&BigFixed> for BigFixed {
    fn bitxor_assign(&mut self, other: &BigFixed) {
        let low = min(self.position, other.position);
        let high = max(self.position_high(), other.position_high());
        self.expand_range(low, high);
        for position in low..high {
            self.set(self.get(position) ^ other.get(position), position);
        }
    }
}

// DerefMut is only for smart pointers and that's not what this is

// Div is special -- deal with it later

// DivAssign ^

// Drop ^^

// Fn* -- not relevant
*/
impl ops::Index<isize> for BigFixed {
    type Output = Digit;
    fn index(&self, position: isize) -> &Digit {
        let shifted = position - self.position;
        if shifted >= self.body.len() as isize {
            &self.head
        } else if shifted >= 0 {
            &self.body[position as usize]
        } else {
            println!("shifted {} position {} val {}", shifted, position, self.tail[shifted]);
            &self.tail[shifted]
        }
    }
}

impl ops::IndexMut<isize> for BigFixed {
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
Neg

The unary negation operator -.
Not

The unary logical negation operator !.
RangeBounds

RangeBounds is implemented by Rust’s built-in range types, produced by range syntax like .., a.., ..b, ..=c, d..e, or f..=g.
Rem

The remainder operator %.
RemAssign

The remainder assignment operator %=.
Shl

The left shift operator <<. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust’s type checker has special handling for _ << _, setting the result type for integer operations to the type of the left-hand-side operand. This means that though a << b and a.shl(b) are one and the same from an evaluation standpoint, they are different when it comes to type inference.
ShlAssign

The left shift assignment operator <<=.
Shr

The right shift operator >>. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust’s type checker has special handling for _ >> _, setting the result type for integer operations to the type of the left-hand-side operand. This means that though a >> b and a.shr(b) are one and the same from an evaluation standpoint, they are different when it comes to type inference.
ShrAssign

The right shift assignment operator >>=.
Sub

The subtraction operator -.
SubAssign
*/
