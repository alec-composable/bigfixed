/*
    Digit is a generalizaiton of unsigned integer arithmetic. Basically everything that the u* natives have in common.

    Digit captures the properties of the numbers D = {0, 1, 2, ..., 2^n-1} with cyclic (wrapping) arithmetic.
    Equality and ordering respect the number representatives listed above: 0 through 2^n-1.

    Arithmetic and overflow are well-defined. If x,y are two Digits (elements of D) then
        x+y = a*2^n + b
    where a,b are also Digits. We call a the overflow and b the result. The same goes for multiplication.

    Subtraction is defined via negation where -a = 2^n-a (except -0 = 0); this operation is closed on the set D.
    
    Division and remainder are defined as integer division: dividing x by the nonzero Digit y gives Digits q,r where
        x = qy + r,
        0 <= r < y.
    Here q is the quotient (div) and r the remainder (rem).
    
    We choose D to have 2^n elements so that we can talk about binary representations. With this comes all
    the bitwise operations: BitAnd, BitOr, BitXor, Not, Shl, Shr. Right shifting pads with 0s from the left.

    There are special constants with each value of n defining D. The numbers 0 and 1 are self explanatory.
    The number 2^n-1, the maximal element of D, is called ALLONES. Its binary representation is 11111...111.
    Another useful constant is 2^(n-1) with binary representation 10000....000. This is called GREATESTBIT.

    Any Digit can be cast back and forth from all the native u* types. This is a direct bit cast just like
    how the u* types convert between each other using the 'as' keyword. Digits can be cast to other Digits too.

    Because we can't implement existing traits on native u* types, some of the traits are instead hard coded.

    The type u128 is not made into a Digit because there is no u256 for evaluating the full operations.
*/

use paste::paste;

use core::{cmp::{PartialEq, Eq}, fmt,
    ops::{
        Add, AddAssign,
        BitAnd, BitAndAssign,
        BitOr, BitOrAssign,
        BitXor, BitXorAssign,
        Div, DivAssign,
        Mul, MulAssign,
        Not,
        Rem, RemAssign,
        Shl, ShlAssign,
        Shr, ShrAssign,
        Sub, SubAssign
    }
};

pub trait Digit:
    Clone + Copy + PartialEq + Eq + PartialOrd
    + Add<Self, Output = Self> + AddAssign<Self>
    + BitAnd<Self, Output = Self> + BitAndAssign<Self>
    + BitOr<Self, Output = Self> + BitOrAssign<Self>
    + BitXor<Self, Output = Self> + BitXorAssign<Self>
    + Div<Self, Output = Self> + DivAssign<Self>
    + Mul<Self, Output = Self> + MulAssign<Self>
    + Not<Output = Self>
    + Rem<Self, Output = Self> + RemAssign<Self>
    + Shl<Self, Output = Self> + ShlAssign<Self>
    + Shr<Self, Output = Self> + ShrAssign<Self>
    + Shl<usize, Output = Self> + ShlAssign<usize>
    + Shr<usize, Output = Self> + ShrAssign<usize>
    + Sub<Self, Output = Self> + SubAssign<Self>
    + fmt::Display + fmt::Debug + fmt::Octal + fmt::LowerHex + fmt::UpperHex + fmt::Binary
{
    const DIGITBITS: usize;
    const DIGITBYTES: usize = Self::DIGITBITS / 8;

    const ZERO: Self;
    const ONE: Self;
    const GREATESTBIT: Self;
    const ALLONES: Self;

    fn from_le_bytes(bytes: &[u8]) -> Self;
    fn to_le_bytes(&self) -> Vec<u8>;

    // these combined calls evaluate the result and overflow simultaneously, storing the results in the respective mutable values
    fn combined_add(x: Self, y: Self, result: &mut Self, carry: &mut Self);
    fn combined_mul(x: Self, y: Self, result: &mut Self, carry: &mut Self);

    fn neg(&self) -> Self;

    fn leading_zeros(&self) -> usize;

    fn from_u8(x: u8) -> Self;
    fn from_u16(x: u16) -> Self;
    fn from_u32(x: u32) -> Self;
    fn from_u64(x: u64) -> Self;
    fn from_u128(x: u128) -> Self;
    fn from_usize(x: usize) -> Self;

    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_u32(&self) -> u32;
    fn to_u64(&self) -> u64;
    fn to_u128(&self) -> u128;
    fn to_usize(&self) -> usize;
}

macro_rules! build_digit {
    ($bits: expr) => {
        paste! {
            impl Digit for [<u $bits>] {
                const DIGITBITS: usize = [<u $bits>]::BITS as usize;
            
                const ZERO: [<u $bits>] = 0;
                const ONE: [<u $bits>] = 1;
                const GREATESTBIT: [<u $bits>] = !((!0) >> 1);
                const ALLONES: [<u $bits>] = !0;
            
                fn from_le_bytes(bytes: &[u8]) -> Self {
                    let mut right_bytes = [0; Self::DIGITBYTES];
                    right_bytes.copy_from_slice(bytes);
                    <[<u $bits>]>::from_le_bytes(right_bytes)
                }
                fn to_le_bytes(&self) -> Vec<u8> {
                    <[<u $bits>]>::to_le_bytes(*self).into()
                }
            
                // these combined calls evaluate the result and overflow simultaneously, storing the results in the respective mutable values
                fn combined_add(x: Self, y: Self, result: &mut Self, carry: &mut Self) {
                    let sum = (x as u16) + (y as u16);
                    *result = sum as Self;
                    *carry = (sum >> 8) as Self;
                }
                fn combined_mul(x: Self, y: Self, result: &mut Self, carry: &mut Self) {
                    let prod = (x as u16) * (y as u16);
                    *result = prod as Self;
                    * carry = (prod >> 8) as Self;
                }
            
                fn neg(&self) -> Self {
                    !self.wrapping_add(1)
                }
            
                fn leading_zeros(&self) -> usize {
                    <[<u $bits>]>::leading_zeros(*self) as usize
                }
            
                fn from_u8(x: u8) -> Self {
                    x as Self
                }
                fn from_u16(x: u16) -> Self {
                    x as Self
                }
                fn from_u32(x: u32) -> Self {
                    x as Self
                }
                fn from_u64(x: u64) -> Self {
                    x as Self
                }
                fn from_u128(x: u128) -> Self {
                    x as Self
                }
                fn from_usize(x: usize) -> Self {
                    x as Self
                }
            
                fn to_u8(&self) -> u8 {
                    *self as u8
                }
                fn to_u16(&self) -> u16 {
                    *self as u16
                }
                fn to_u32(&self) -> u32 {
                    *self as u32
                }
                fn to_u64(&self) -> u64 {
                    *self as u64
                }
                fn to_u128(&self) -> u128 {
                    *self as u128
                }
                fn to_usize(&self) -> usize {
                    *self as usize
                }
            }
        }                
    };
}

build_digit!(8);
build_digit!(16);
build_digit!(32);
build_digit!(64);
build_digit!(size);
