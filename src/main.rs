use numbers::{digit::*, Index, Cutoff, CutsOff, BigFixed};

use fastrand;

pub fn main() {
    let mut a = BigFixed::from(1125912791875585u64);
    a.head = ALLONES;
    a.position = 1isize.into();
    println!("{}", a);
    for i in -3..7isize {
        for j in -1..6isize {
            let mut b = a.clone();
            let cutoff = Cutoff{
                fixed: if i == -3 {None} else {Some(Index::from(i))},
                floating: if j == -1 {None} else {Some(Index::from(j))}
            };
            b.cutoff(cutoff);
            println!("{}\t{}", cutoff, b);
        }
    }
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

// to get the compiler to shut up about unused imports

pub fn trivial_digit() -> Digit {
    0
}

pub fn trivial_index() -> Index {
    Index::ZERO
}

pub fn trivial_bigfixed() -> BigFixed {
    BigFixed::from(0)
}
