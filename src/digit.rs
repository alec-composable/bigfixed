// Digit specifies u* arithmetic (wrapping) from one of the native integer types. It could be hard coded as u32 or u64 but
// the smaller types u16/u8 are easier to work with while developing and testing. Hence which one is in use is decided via this macro.

use paste::paste;

macro_rules! build_digit {
    ($bits: expr, $double_bits: expr) => {
        paste!{
            pub const DIGITBITS: usize = $bits;
            pub const DIGITBYTES: usize = DIGITBITS / 8;
            pub type Digit = [<u $bits>];
            pub type SignedDigit = [<i $bits>];
            
            pub const DOUBLEBITS: usize = $double_bits;
            pub const DOUBLEBYTES: usize = 2*DIGITBYTES;
            pub type DoubleDigit = [<u $double_bits>];
            pub type SignedDoubleDigit = [<i $double_bits>];

            pub const ALLONES: Digit = (-1 as SignedDigit) as Digit;
            pub const GREATESTBIT: Digit = 1 << (DIGITBITS - 1);

            pub fn digit_from_bytes(bytes: &[u8]) -> Digit {
                Digit::from_le_bytes(bytes.try_into().unwrap())
            }
            
            #[macro_export]
            macro_rules! add {
                ($a: expr, $b: expr, $result: expr, $carry: expr) => {
                    let res = ($a as DoubleDigit) + ($b as DoubleDigit);
                    $result = res as Digit;
                    $carry = (res >> DIGITBITS) as Digit;
                };
                ($a: expr, $b: expr, $result: expr) => {
                    let res = ($a as Double) + ($b as Double);
                    $result = res as Digit;
                };
            }
            
            pub(crate) use add;
            
            #[macro_export]
            macro_rules! mul {
                ($a: expr, $b: expr, $result: expr, $carry: expr) => {
                    let res = ($a as DoubleDigit) * ($b as DoubleDigit);
                    $result = res as Digit;
                    $carry = (res >> DIGITBITS) as Digit;
                };
                ($a: expr, $b: expr, $result: expr) => {
                    let res = ($a as Double) * ($b as Double);
                    $result = res as Digit;
                };
            }
            
            pub(crate) use mul;

            #[macro_export]
            macro_rules! div {
                ($dividend_high: expr, $dividend_low: expr, $divisor: expr, $quot: expr) => {
                    let dividend = (($dividend_high as DoubleDigit) << DIGITBITS) | ($dividend_low as DoubleDigit);
                    let divisor = $divisor as DoubleDigit;
                    $quot = (dividend / divisor) as Digit;
                };
            }
            pub(crate) use div;
        }
    };
}

build_digit!(16, 32);

#[macro_export]
macro_rules! binary_formatter {
    () => {
        "{:#018b}" // digit bits + 2 for 0b
    };
}

pub(crate) use binary_formatter;
