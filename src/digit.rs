use paste::paste;

// This is better but it can still be improved. It would be really nice to only have to call build_digit!(2) to get u16 digits.

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
