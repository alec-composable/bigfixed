// Digit specifies u* arithmetic (wrapping) from one of the native integer types. It could be hard coded as u32 or u64 but
// the smaller types u16/u8 are easier to work with while developing and testing. Hence which one is in use is decided via this macro.

use paste::paste;
use num_traits::PrimInt;
use core::{cmp::{PartialEq, Eq, Ordering}, fmt::{Debug},
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
    }
};
use crate::op_to_op_assign;

pub trait Arithmetic<Other, Out>: 
    From<Other> + Into<Other> + PartialEq<Other> + PartialOrd<Other>
    + Add<Other, Output = Out> + AddAssign<Other>
    + BitAnd<Other, Output = Out> + BitAndAssign<Other>
    + BitOr<Other, Output = Out> + BitOrAssign<Other>
    + BitXor<Other, Output = Out> + BitXorAssign<Other>
    + Mul<Other, Output = Out> + MulAssign<Other>
    + Neg + Not
    + Shl<Other, Output = Out> + ShlAssign<Other>
    + Shr<Other, Output = Out> + ShrAssign<Other>
    + Sub<Other, Output = Out> + SubAssign<Other>
{}

pub trait Digit: 
    Eq + Debug + Clone + Copy
    + Arithmetic<u8, Self> + Arithmetic<u16, Self> + Arithmetic<u32, Self> + Arithmetic<u64, Self> + Arithmetic<u128, Self> + Arithmetic<usize, Self>
    + Arithmetic<Self, Self>
{
    const DIGITBITS: usize;
    const DIGITBYTES: usize;
    type Digit: PrimInt;
    type SignedDigit: PrimInt;

    const DOUBLEBITS: usize;
    const DOUBLEBYTES: usize;
    type DoubleDigit: PrimInt;
    type SignedDoubleDigit: PrimInt;

    const ALLONES: Self::Digit;
    const GREATESTBIT: Self::Digit;

    fn digit_from_bytes(bytes: &[u8]) -> Self::Digit;

    fn add(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit);
    fn add_full(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit, carry: &mut Self::Digit);

    fn mul(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit);
    fn mul_full(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit, carry: &mut Self::Digit);

    fn div(dividend_high: &Self::Digit, dividend_low: &Self::Digit, denominator: &Self::Digit, quotient: &mut Self::Digit);
}

macro_rules! build_digit {
    ($bits: expr, $double_bits: expr) => {
        paste!{
            #[derive(PartialEq, Eq, Debug, Clone, Copy)]
            pub struct [<Digit $bits>] {
                value: [<u $bits>]
            }

            impl Digit for [<Digit $bits>] {
                const DIGITBITS: usize = $bits;
                const DIGITBYTES: usize = Self::DIGITBITS >> 3;
                type Digit = [<u $bits>];
                type SignedDigit = [<i $bits>];
                
                const DOUBLEBITS: usize = $double_bits;
                const DOUBLEBYTES: usize = 2 * Self::DIGITBYTES;
                type DoubleDigit = [<u $double_bits>];
                type SignedDoubleDigit = [<i $double_bits>];

                const ALLONES: Self::Digit = (-1 as Self::SignedDigit) as Self::Digit;
                const GREATESTBIT: Self::Digit = 1 << (Self::DIGITBITS - 1);

                fn digit_from_bytes(bytes: &[u8]) -> Self::Digit {
                    Self::Digit::from_le_bytes(bytes.try_into().unwrap())
                }
                
                fn add(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit) {
                    let res = (*a as Self::DoubleDigit) + (*b as Self::DoubleDigit);
                    *result = res as Self::Digit;
                }
                fn add_full(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit, carry: &mut Self::Digit) {
                    let res = (*a as Self::DoubleDigit) + (*b as Self::DoubleDigit);
                    *result = res as Self::Digit;
                    *carry = (res >> Self::DIGITBITS) as Self::Digit;
                }
                
                fn mul(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit) {
                    let res = (*a as Self::DoubleDigit) * (*b as Self::DoubleDigit);
                    *result = res as Self::Digit;
                }
                fn mul_full(a: &Self::Digit, b: &Self::Digit, result: &mut Self::Digit, carry: &mut Self::Digit) {
                    let res = (*a as Self::DoubleDigit) * (*b as Self::DoubleDigit);
                    *result = res as Self::Digit;
                    *carry = (res >> Self::DIGITBITS) as Self::Digit;
                }
                
                fn div(dividend_high: &Self::Digit, dividend_low: &Self::Digit, divisor: &Self::Digit, quotient: &mut Self::Digit) {
                    let dividend = ((*dividend_high as Self::DoubleDigit) << Self::DIGITBITS) | (*dividend_low as Self::DoubleDigit);
                    let divisor = *divisor as Self::DoubleDigit;
                    *quotient = (dividend / divisor) as Self::Digit;
                }
            }

            converters!($bits);
            comparisons!($bits);
            arithmetics!($bits);

            impl Arithmetic<[<Digit $bits>], [<Digit $bits>]> for [<Digit $bits>] {}
        }
    };
}

macro_rules! converters {
    ($digit_bits: expr, $native: expr) => {
        paste!{
            impl From<$native> for [<Digit $digit_bits>] {
                fn from(x: $native) -> [<Digit $digit_bits>] {
                    [<Digit $digit_bits>] {
                        value: x as [<u $digit_bits>]
                    }
                }
            }
            
            impl From<[<Digit $digit_bits>]> for $native {
                fn from(x: [<Digit $digit_bits>]) -> $native {
                    x.value as $native
                }
            }
            
            impl From<&[<Digit $digit_bits>]> for $native {
                fn from(x: &[<Digit $digit_bits>]) -> $native {
                    x.value as $native
                }
            }
        }
    };
    ($digit_bits: expr) => {
        converters!($digit_bits, u8);
        converters!($digit_bits, u16);
        converters!($digit_bits, u32);
        converters!($digit_bits, u64);
        converters!($digit_bits, u128);
        converters!($digit_bits, usize);
    }
}

macro_rules! comparisons {
    ($digit_bits: expr, $native: expr) => {
        paste!{
            impl PartialEq<$native> for [<Digit $digit_bits>] {
                fn eq(&self, other: &$native) -> bool {
                    *self == [<Digit $digit_bits>]::from(*other)
                }
            }
            impl PartialOrd<$native> for [<Digit $digit_bits>] {
                fn partial_cmp(&self, other: &$native) -> Option<Ordering> {
                    self.value.partial_cmp(&(*other as [<u $digit_bits>]))
                }
            }
            
        }
    };
    ($digit_bits: expr) => {
        paste! {
            comparisons!($digit_bits, u8);
            comparisons!($digit_bits, u16);
            comparisons!($digit_bits, u32);
            comparisons!($digit_bits, u64);
            comparisons!($digit_bits, u128);
            comparisons!($digit_bits, usize);

            impl PartialOrd for [<Digit $digit_bits>] {
                fn partial_cmp(&self, other: &[<Digit $digit_bits>]) -> Option<Ordering> {
                    self.value.partial_cmp(&other.value)
                }
            }
        }
    }
}

macro_rules! arithmetics {
    ($digit_bits: expr, $native: ty, $op: path, $op_fn_name: ident, $op_call_fn_name: ident, $op_assign: path, $op_assign_fn_name: ident) => {
        paste! {
            impl $op<&$native> for &[<Digit $digit_bits>] {
                type Output = [<Digit $digit_bits>];
                fn $op_fn_name(self, other: &$native) -> [<Digit $digit_bits>] {
                    [<Digit $digit_bits>] {
                        value: self.value.$op_call_fn_name(*other as [<u $digit_bits>])
                    }
                }
            }

            op_to_op_assign!(
                $op, $op_fn_name,
                $op_assign, $op_assign_fn_name,
                [<Digit $digit_bits>], $native,
                [<Digit $digit_bits>]
            );
        }
    };
    ($digit_bits: expr, $op: path, $op_fn_name: ident, $op_assign: path, $op_assign_fn_name: ident) => {
        paste! {
            impl $op<&[<Digit $digit_bits>]> for &[<Digit $digit_bits>] {
                type Output = [<Digit $digit_bits>];
                fn $op_fn_name(self, other: &[<Digit $digit_bits>]) -> [<Digit $digit_bits>] {
                    [<Digit $digit_bits>] {
                        value: self.value + <[<u $digit_bits>]>::from(other)
                    }
                }
            }

            op_to_op_assign!(
                $op, $op_fn_name,
                $op_assign, $op_assign_fn_name,
                [<Digit $digit_bits>], [<Digit $digit_bits>],
                [<Digit $digit_bits>]
            );
        }
    };
    ($digit_bits: expr, $native: expr) => {
        paste!{
            arithmetics!($digit_bits, $native, Add, add, wrapping_add, AddAssign, add_assign);
            arithmetics!($digit_bits, $native, BitAnd, bitand, bitand, BitAndAssign, bitand_assign);
            arithmetics!($digit_bits, $native, BitOr, bitor, bitor, BitOrAssign, bitor_assign);
            arithmetics!($digit_bits, $native, BitXor, bitxor, bitxor, BitXorAssign, bitxor_assign);
            arithmetics!($digit_bits, $native, Mul, mul, wrapping_mul, MulAssign, mul_assign);
            arithmetics!($digit_bits, $native, Shl, shl, shl, ShlAssign, shl_assign);
            arithmetics!($digit_bits, $native, Shr, shr, shr, ShrAssign, shr_assign);
            arithmetics!($digit_bits, $native, Sub, sub, wrapping_sub, SubAssign, sub_assign);
            impl Arithmetic<$native, [<Digit $digit_bits>]> for [<Digit $digit_bits>] {}
        }
    };
    ($digit_bits: expr) => {
        paste! {
            impl Not for [<Digit $digit_bits>] {
                type Output = [<Digit $digit_bits>];
                fn not(self) -> [<Digit $digit_bits>] {
                    [<Digit $digit_bits>] {
                        value: !self.value
                    }
                }
            }
            impl Neg for [<Digit $digit_bits>] {
                type Output = [<Digit $digit_bits>];
                fn neg(self) -> [<Digit $digit_bits>] {
                    [<Digit $digit_bits>] {
                        value: (-(self.value as [<i $digit_bits>])) as [<u $digit_bits>]
                    }
                }
            }
            arithmetics!($digit_bits, u8);
            arithmetics!($digit_bits, u16);
            arithmetics!($digit_bits, u32);
            arithmetics!($digit_bits, u64);
            arithmetics!($digit_bits, u128);
            arithmetics!($digit_bits, usize);
            arithmetics!($digit_bits, Add, add, AddAssign, add_assign);
            arithmetics!($digit_bits, BitAnd, bitand, BitAndAssign, bitand_assign);
            arithmetics!($digit_bits, BitOr, bitor, BitOrAssign, bitor_assign);
            arithmetics!($digit_bits, BitXor, bitxor, BitXorAssign, bitxor_assign);
            arithmetics!($digit_bits, Mul, mul, MulAssign, mul_assign);
            arithmetics!($digit_bits, Shl, shl, ShlAssign, shl_assign);
            arithmetics!($digit_bits, Shr, shr, ShrAssign, shr_assign);
            arithmetics!($digit_bits, Sub, sub, SubAssign, sub_assign);
        }
    }
}

build_digit!(8, 16);
build_digit!(16, 32);
build_digit!(32, 64);
build_digit!(64, 128);
