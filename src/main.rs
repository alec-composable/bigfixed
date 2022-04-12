use numbers::digit::*;
use numbers::BigFixed;

use fastrand;

pub fn main() {
    for _i in 0..100000 {
        let shift = fastrand::isize(..) / 2;
        let a = fastrand::i16(..) as i32;
        let b = fastrand::i16(..) as i32;
        let x = BigFixed::from(a).shift(shift);
        let y = BigFixed::from(b).shift(-shift);
        assert_eq!(
            <i64>::from(&BigFixed::from(a * b)),
            <i64>::from(&(&x * &y)),
            "a {} {} b {} {} ab {} {} {}", a, x, b, y, a*b, BigFixed::from(a * b), &x * &y);
    }
    println!("all done");
}

pub fn rand() -> BigFixed {
    BigFixed::from(fastrand::i128(..))
}

pub fn bit_test() {
    println!("i8\t0\t{}", BigFixed::from(0i8));
    println!("i8\t1\t{}", BigFixed::from(1i8));
    println!("i8\t-1\t{}", BigFixed::from(-1i8));
    println!("u8\t0\t{}", BigFixed::from(0u8));
    println!("u8\t1\t{}", BigFixed::from(1u8));
    println!("u8\t-1\t{}", BigFixed::from(-1i8 as u8));
    println!("i16\t0\t{}", BigFixed::from(0i16));
    println!("i16\t1\t{}", BigFixed::from(1i16));
    println!("i16\t-1\t{}", BigFixed::from(-1i16));
    println!("u16\t0\t{}", BigFixed::from(0u16));
    println!("u16\t1\t{}", BigFixed::from(1u16));
    println!("u16\t-1\t{}", BigFixed::from(-1i16 as u16));
    println!("i32\t0\t{}", BigFixed::from(0i32));
    println!("i32\t1\t{}", BigFixed::from(1i32));
    println!("i32\t-1\t{}", BigFixed::from(-1i32));
    println!("u32\t0\t{}", BigFixed::from(0u32));
    println!("u32\t1\t{}", BigFixed::from(1u32));
    println!("u32\t-1\t{}", BigFixed::from(-1i32 as u32));
    println!("i64\t0\t{}", BigFixed::from(0i64));
    println!("i64\t1\t{}", BigFixed::from(1i64));
    println!("i64\t-1\t{}", BigFixed::from(-1i64));
    println!("u64\t0\t{}", BigFixed::from(0u64));
    println!("u64\t1\t{}", BigFixed::from(1u64));
    println!("u64\t-1\t{}", BigFixed::from(-1i64 as u64));
    println!("i128\t0\t{}", BigFixed::from(0i128));
    println!("i128\t1\t{}", BigFixed::from(1i128));
    println!("i128\t-1\t{}", BigFixed::from(-1i128));
    println!("u128\t0\t{}", BigFixed::from(0u128));
    println!("u128\t1\t{}", BigFixed::from(1u128));
    println!("u128\t-1\t{}", BigFixed::from(-1i128 as u128));
}

pub fn trivial_digit() -> Digit {
    0
}

pub fn trivial_bigfixed() -> BigFixed {
    BigFixed::from(0)
}
