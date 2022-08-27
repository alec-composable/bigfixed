use crate::digit::*;

use core::{
    cmp::{
        PartialEq,
        Eq,
        Ordering,
    },
    convert::From,
    iter::repeat,
    fmt,
    ops::{
        BitAnd, BitAndAssign,
        BitOr, BitOrAssign,
        BitXor, BitXorAssign,
        Div, DivAssign,
        Not,
        Rem, RemAssign,
        Shl, ShlAssign,
        Shr, ShrAssign,
    }
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DoubleDigit<D: Digit> {
    pub lesser: D,
    pub greater: D,
}

impl<D: Digit> PartialOrd for DoubleDigit<D> {
    fn partial_cmp(&self, other: &DoubleDigit<D>) -> Option<Ordering> {
        match self.greater.partial_cmp(&other.greater)? {
            Ordering::Equal => self.lesser.partial_cmp(&other.lesser),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

impl<D: Digit> BitAnd<DoubleDigit<D>> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn bitand(self, other: DoubleDigit<D>) -> DoubleDigit<D> {
        DoubleDigit {
            greater: self.greater & other.greater,
            lesser: self.lesser & other.lesser,
        }
    }
}

impl<D: Digit> BitAndAssign<DoubleDigit<D>> for DoubleDigit<D> {
    fn bitand_assign(&mut self, other: DoubleDigit<D>) {
        self.greater &= other.greater;
        self.lesser &= other.lesser;
    }
}

impl<D: Digit> BitOr<DoubleDigit<D>> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn bitor(self, other: DoubleDigit<D>) -> DoubleDigit<D> {
        DoubleDigit {
            greater: self.greater | other.greater,
            lesser: self.lesser | other.lesser,
        }
    }
}

impl<D: Digit> BitOrAssign<DoubleDigit<D>> for DoubleDigit<D> {
    fn bitor_assign(&mut self, other: DoubleDigit<D>) {
        self.greater |= other.greater;
        self.lesser |= other.lesser;
    }
}

impl<D: Digit> BitXor<DoubleDigit<D>> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn bitxor(self, other: DoubleDigit<D>) -> DoubleDigit<D> {
        DoubleDigit {
            greater: self.greater ^ other.greater,
            lesser: self.lesser ^ other.lesser,
        }
    }
}

impl<D: Digit> BitXorAssign<DoubleDigit<D>> for DoubleDigit<D> {
    fn bitxor_assign(&mut self, other: DoubleDigit<D>) {
        self.greater ^= other.greater;
        self.lesser ^= other.lesser;
    }
}

impl<D: Digit> Div<DoubleDigit<D>> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn div(self, other: DoubleDigit<D>) -> DoubleDigit<D> {
        // placeholder
        self | other
    }
}

impl<D: Digit> DivAssign<DoubleDigit<D>> for DoubleDigit<D> {
    fn div_assign(&mut self, other: DoubleDigit<D>) {
        *self |= other;
    }
}

impl<D: Digit> Not for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn not(self) -> DoubleDigit<D> {
        DoubleDigit {
            greater: !self.greater,
            lesser: !self.lesser,
        }
    }
}

impl<D: Digit> Rem<DoubleDigit<D>> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn rem(self, other: DoubleDigit<D>) -> DoubleDigit<D> {
        // placeholder
        self | other
    }
}

impl<D: Digit> RemAssign<DoubleDigit<D>> for DoubleDigit<D> {
    fn rem_assign(&mut self, other: DoubleDigit<D>) {
        // placeholder
        *self |= other;
    }
}

impl<D: Digit> Shl<DoubleDigit<D>> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn shl(self, other: DoubleDigit<D>) -> DoubleDigit<D> {
        if other.greater != D::ZERO || other.lesser >= D::from_usize(usize::MAX) {
            DoubleDigit {
                greater: D::ZERO,
                lesser: D::ZERO,
            }
        } else {
            self << other.lesser.to_usize()
        }
    }
}

impl<D: Digit> ShlAssign<DoubleDigit<D>> for DoubleDigit<D> {
    fn shl_assign(&mut self, other: DoubleDigit<D>) {
        *self = *self << other;
    }
}

impl<D: Digit> Shr<DoubleDigit<D>> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn shr(self, other: DoubleDigit<D>) -> DoubleDigit<D> {
        if other.greater != D::ZERO || other.lesser >= D::from_usize(usize::MAX) {
            DoubleDigit {
                greater: D::ZERO,
                lesser: D::ZERO,
            }
        } else {
            self >> other.lesser.to_usize()
        }
    }
}

impl<D: Digit> ShrAssign<DoubleDigit<D>> for DoubleDigit<D> {
    fn shr_assign(&mut self, other: DoubleDigit<D>) {
        *self = *self >> other;
    }
}

impl<D: Digit> Shl<usize> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn shl(self, other: usize) -> DoubleDigit<D> {
        if other == 0 {
            self.clone()
        } else if other < D::DIGITBITS {
            DoubleDigit {
                greater: (self.greater << other) | (self.lesser >> (D::DIGITBITS - other)),
                lesser: self.lesser << other,
            }
        } else if other < 2*D::DIGITBITS {
            DoubleDigit {
                greater: self.lesser << (other - D::DIGITBITS),
                lesser: D::ZERO,
            }
        } else {
            DoubleDigit::ZERO
        }
    }
}

impl<D: Digit> ShlAssign<usize> for DoubleDigit<D> {
    fn shl_assign(&mut self, other: usize) {
        *self = *self << other;
    }
}

impl<D: Digit> Shr<usize> for DoubleDigit<D> {
    type Output = DoubleDigit<D>;
    fn shr(self, other: usize) -> DoubleDigit<D> {
        if other == 0 {
            self.clone()
        } else if other < D::DIGITBITS {
            DoubleDigit {
                greater: self.greater >> other,
                lesser: (self.lesser >> other) | (self.greater << (D::DIGITBITS - other)),
            }
        } else if other < 2*D::DIGITBITS {
            DoubleDigit {
                greater: D::ZERO,
                lesser: self.greater >> (other - D::DIGITBITS),
            }
        } else {
            DoubleDigit::ZERO
        }
    }
}

impl<D: Digit> ShrAssign<usize> for DoubleDigit<D> {
    fn shr_assign(&mut self, other: usize) {
        *self = *self >> other;
    }
}

impl<D: Digit> fmt::Display for DoubleDigit<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.greater, self.lesser)
    }
}

impl<D: Digit> fmt::Debug for DoubleDigit<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?})", self.greater, self.lesser)
    }
}

impl<D: Digit> fmt::Octal for DoubleDigit<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:o},{:o})", self.greater, self.lesser)
    }
}

impl<D: Digit> fmt::LowerHex for DoubleDigit<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:x},{:x})", self.greater, self.lesser)
    }
}

impl<D: Digit> fmt::UpperHex for DoubleDigit<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:X},{:X})", self.greater, self.lesser)
    }
}

impl<D: Digit> fmt::Binary for DoubleDigit<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_binary(f)
    }
}

impl<D: Digit> Digit for DoubleDigit<D> {
    const DIGITBITS: usize = D::DIGITBITS * 2;

    const ZERO: Self = Self {
        greater: D::ZERO,
        lesser: D::ZERO,
    };
    const ONE: Self = Self {
        greater: D::ZERO,
        lesser: D::ONE,
    };
    const GREATESTBIT: Self = Self {
        greater: D::GREATESTBIT,
        lesser: D::ZERO,
    };
    const ALLONES: Self = Self {
        greater: D::ALLONES,
        lesser: D::ALLONES,
    };

    fn from_le_bytes<'a, I: Iterator<Item = &'a u8> + Clone>(bytes: I) -> Self {
        
        Self {
            lesser: D::from_le_bytes(bytes.clone().chain(repeat(&0))),
            greater: D::from_le_bytes(bytes.chain(repeat(&0)).skip(D::DIGITBYTES))
        }
    }
    fn to_le_bytes(&self) -> Vec<u8> {
        self.lesser.to_le_bytes().into_iter().chain(
            self.greater.to_le_bytes().into_iter()
        ).collect()
    }

    // these combined calls evaluate the result and overflow simultaneously, storing the results in the respective mutable values
    fn combined_add(x: Self, y: Self, result: &mut Self, carry: &mut Self) {
        let mut lesser = D::ZERO;
        let mut sub_carry = D::ZERO;
        D::combined_add(x.lesser, y.lesser, &mut lesser, &mut sub_carry);
        let mut greater = D::ZERO;
        D::combined_add(x.greater, sub_carry, &mut greater, &mut sub_carry);
        let mut carried = sub_carry != D::ZERO;
        D::combined_add(greater, y.greater, &mut greater, &mut sub_carry);
        carried = carried || sub_carry != D::ZERO;
        *result = Self {
            greater: greater,
            lesser: lesser,
        };
        *carry = if carried {
            Self::ONE
        } else {
            Self::ZERO
        };
    }
    fn combined_mul(x: Self, y: Self, result: &mut Self, carry: &mut Self) {
        // (a+b)(A+B)=aA+aB+bA+AB
        //   x *  y : f  o  i  l , f == carry, l == result
        // multiply least pair into result
        D::combined_mul(x.lesser, y.lesser, &mut result.lesser, &mut result.greater);
        // multiply greatest pait into carry
        D::combined_mul(x.greater, y.greater, &mut carry.lesser, &mut carry.greater);

        // cross terms
        let mut il = D::ZERO;
        let mut ig = D::ZERO;
        D::combined_mul(x.lesser, y.greater, &mut il, &mut ig);
        let mut ol = D::ZERO;
        let mut og = D::ZERO;
        D::combined_mul(x.greater, y.lesser, &mut ol, &mut og);
        
        // add cross terms in
        let mut c = D::ZERO;
        D::combined_add(result.greater, il, &mut result.greater, &mut c);
        if c != D::ZERO {
            Self::wrapping_increment(*carry, carry);
        }
        D::combined_add(result.greater, ol, &mut result.greater, &mut c);
        if c != D::ZERO {
            Self::wrapping_increment(*carry, carry);
        }
        D::combined_add(carry.lesser, ig, &mut carry.lesser, &mut c);
        if c != D::ZERO {
            D::wrapping_increment(carry.greater, &mut carry.greater);
        }
        D::combined_add(carry.lesser, og, &mut carry.lesser, &mut c);
        if c != D::ZERO {
            D::wrapping_increment(carry.greater, &mut carry.greater);
        }
    }

    fn wrapping_increment(x: Self, result: &mut Self) {
        let mut lesser = D::ZERO;
        let mut carry = D::ZERO;
        D::combined_add(x.lesser, D::ONE, &mut lesser, &mut carry);
        let mut greater = D::ZERO;
        D::combined_add(x.greater, carry, &mut greater, &mut carry);
        *result = Self {
            greater: greater,
            lesser: lesser,
        }
    }

    fn neg(&self) -> Self {
        let mut res = !*self;
        Self::wrapping_increment(res, &mut res);
        res
    }

    fn leading_zeros(&self) -> Index {
        let g0 = self.greater.leading_zeros();
        if g0 == D::DIGITBITSI {
            g0 + self.lesser.leading_zeros()
        } else {
            g0
        }
    }
    fn leading_ones(&self) -> Index {
        let g1 = self.greater.leading_ones();
        if g1 == D::DIGITBITSI {
            g1 + self.lesser.leading_ones()
        } else {
            g1
        }
    }
    fn trailing_zeros(&self) -> Index {
        let l0 = self.lesser.trailing_zeros();
        if l0 == D::DIGITBITSI {
            l0 + self.greater.trailing_zeros()
        } else {
            l0
        }
    }
    fn trailing_ones(&self) -> Index {
        let l1 = self.lesser.trailing_ones();
        if l1 == D::DIGITBITSI {
            l1 + self.greater.trailing_ones()
        } else {
            l1
        }
    }

    fn from_u8(x: u8) -> Self {
        Self::from_le_bytes(x.to_le_bytes().iter())
    }
    fn from_u16(x: u16) -> Self {
        Self::from_le_bytes(x.to_le_bytes().iter())
    }
    fn from_u32(x: u32) -> Self {
        Self::from_le_bytes(x.to_le_bytes().iter())
    }
    fn from_u64(x: u64) -> Self {
        Self::from_le_bytes(x.to_le_bytes().iter())
    }
    fn from_u128(x: u128) -> Self {
        Self::from_le_bytes(x.to_le_bytes().iter())
    }
    fn from_usize(x: usize) -> Self {
        Self::from_le_bytes(x.to_le_bytes().iter())
    }

    fn to_u8(&self) -> u8 {
        let mut bytes = self.to_le_bytes();
        bytes.resize(1, 0);
        <u8>::from_le_bytes([bytes[0]])
    }
    fn to_u16(&self) -> u16 {
        let mut bytes = self.to_le_bytes();
        bytes.resize(2, 0);
        <u16>::from_le_bytes([bytes[0], bytes[1]])
    }
    fn to_u32(&self) -> u32 {
        let mut bytes = self.to_le_bytes();
        bytes.resize(4, 0);
        <u32>::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    fn to_u64(&self) -> u64 {
        let mut bytes = self.to_le_bytes();
        bytes.resize(8, 0);
        <u64>::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]])
    }
    fn to_u128(&self) -> u128 {
        let mut bytes = self.to_le_bytes();
        bytes.resize(16, 0);
        <u128>::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]])
    }
    fn to_usize(&self) -> usize {
        self.to_u128() as usize
    }

    fn fmt_binary(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.greater.fmt_binary(f).ok();
        self.lesser.fmt_binary(f)
    }
}

impl<D: Digit> From<u8> for DoubleDigit<D> {
    fn from(x: u8) -> DoubleDigit<D> {
        DoubleDigit::from_u8(x)
    }
}

impl<D: Digit> From<DoubleDigit<D>> for u8 {
    fn from(x: DoubleDigit<D>) -> u8 {
        x.to_u8()
    }
}

impl<D: Digit> From<u16> for DoubleDigit<D> {
    fn from(x: u16) -> DoubleDigit<D> {
        DoubleDigit::from_u16(x)
    }
}

impl<D: Digit> From<DoubleDigit<D>> for u16 {
    fn from(x: DoubleDigit<D>) -> u16 {
        x.to_u16()
    }
}

impl<D: Digit> From<u32> for DoubleDigit<D> {
    fn from(x: u32) -> DoubleDigit<D> {
        DoubleDigit::from_u32(x)
    }
}

impl<D: Digit> From<DoubleDigit<D>> for u32 {
    fn from(x: DoubleDigit<D>) -> u32 {
        x.to_u32()
    }
}

impl<D: Digit> From<u64> for DoubleDigit<D> {
    fn from(x: u64) -> DoubleDigit<D> {
        DoubleDigit::from_u64(x)
    }
}

impl<D: Digit> From<DoubleDigit<D>> for u64 {
    fn from(x: DoubleDigit<D>) -> u64 {
        x.to_u64()
    }
}

impl<D: Digit> From<u128> for DoubleDigit<D> {
    fn from(x: u128) -> DoubleDigit<D> {
        DoubleDigit::from_u128(x)
    }
}

impl<D: Digit> From<DoubleDigit<D>> for u128 {
    fn from(x: DoubleDigit<D>) -> u128 {
        x.to_u128()
    }
}

impl<D: Digit> From<usize> for DoubleDigit<D> {
    fn from(x: usize) -> DoubleDigit<D> {
        DoubleDigit::from_usize(x)
    }
}

impl<D: Digit> From<DoubleDigit<D>> for usize {
    fn from(x: DoubleDigit<D>) -> usize {
        x.to_usize()
    }
}
