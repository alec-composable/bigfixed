// Digit specifies u* arithmetic (wrapping) from one of the native integer types. It could be hard coded as u32 or u64 but
// the smaller types u16/u8 are easier to work with while developing and testing. Hence which one is in use is decided via this macro.

use paste::paste;

pub trait Digit {
    const DIGITBITS: usize;
    const DIGITBYTES: usize;
    type Digit;
    type SignedDigit;

    const DOUBLEBITS: usize;
    const DOUBLEBYTES: usize;
    type DoubleDigit;
    type SignedDoubleDigit;

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
            pub struct [<Digit $bits>] {}

            impl Digit for [<Digit $bits>] {
                const DIGITBITS: usize = $bits;
                const DIGITBYTES: usize = Self::DIGITBITS / 8;
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
        }
    };
}

build_digit!(8, 16);
build_digit!(16, 32);
build_digit!(32, 64);
build_digit!(64, 128);
