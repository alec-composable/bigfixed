// parameters to set. should be a macro? const fn?

/* 8 bit

pub type Digit = u8;

pub const DIGITBYTES: usize = 1;

pub type SignedDigit = i8;

pub type Double = u16;

pub type SignedDouble = i16;

pub fn digit_from_bytes(bytes: &[u8]) -> Digit {
    Digit::from_le_bytes([bytes[0]])
}

// */

//* 16 bit

pub type Digit = u16;

pub const DIGITBYTES: usize = 2;

pub type SignedDigit = i16;

pub type Double = u32;

pub type SignedDouble = i32;

pub fn digit_from_bytes(bytes: &[u8]) -> Digit {
    Digit::from_le_bytes([bytes[0], bytes[1]])
}

// */

/* 32 bit

pub type Digit = u32;

pub const DIGITBYTES: usize = 4;

pub type SignedDigit = i32;

pub type Double = u64;

pub type SignedDouble = i64;

pub fn digit_from_bytes(bytes: &[u8]) -> Digit {
    Digit::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

// */

/* 64 bit

pub type Digit = u64;

pub const DIGITBYTES: usize = 8;

pub type SignedDigit = i64;

pub type Double = u128;

pub type SignedDouble = i128;

pub fn digit_from_bytes(bytes: &[u8]) -> Digit {
    Digit::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}
// */

// done with parameters

pub const DIGITBITS: usize = 8 * DIGITBYTES;

pub const ALLONES: Digit = (-1 as SignedDigit) as Digit;

pub const DOWNCASTMASK: Double = ALLONES as Double;

pub const GREATESTBIT: Digit = 1 << (DIGITBITS - 1);

macro_rules! add {
    ($a: expr, $b: expr, $result: expr, $carry: expr) => {
        let res = ($a as Double) + ($b as Double);
        $result = res as Digit;
        $carry += (res >> DIGITBITS) as Digit;
    };
    ($a: expr, $b: expr, $result: expr) => {
        let res = ($a as Double) + ($b as Double);
        $result = res as Digit;
    };
}

pub(crate) use add;

/*macro_rules! mul {
    ($a: expr, $b: expr, $result: expr, $carry: expr) => {
        let res = ($a as Double) * ($b as Double);
        $result = res as Digit;
        $carry += (res >> DIGITBITS) as Digit;
    };
    ($a: expr, $b: expr, $result: expr) => {
        let res = ($a as Double) * ($b as Double);
        $result = res as Digit;
    };
}

pub(crate) use mul;*/
