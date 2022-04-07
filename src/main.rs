use numbers::digit::*;
//use numbers::Tail;
use numbers::BigFixed;

pub fn main() {
    let a = BigFixed::from(1);
    let b = BigFixed::from(ALLONES);
    println!("a {}", a);
    println!("b {}", b);
    println!("a + b {}", &a + &b);
}

pub fn bit_test() {
    println!("i8 0 {}", BigFixed::from(0i8));
    println!("i8 1 {}", BigFixed::from(1i8));
    println!("i8 -1 {}", BigFixed::from(-1i8));
    println!("u8 0 {}", BigFixed::from(0u8));
    println!("u8 1 {}", BigFixed::from(1u8));
    println!("u8 -1 {}", BigFixed::from(-1i8 as u8));
    println!("i16 0 {}", BigFixed::from(0i16));
    println!("i16 1 {}", BigFixed::from(1i16));
    println!("i16 -1 {}", BigFixed::from(-1i16));
    println!("u16 0 {}", BigFixed::from(0u16));
    println!("u16 1 {}", BigFixed::from(1u16));
    println!("u16 -1 {}", BigFixed::from(-1i16 as u16));
    println!("i32 0 {}", BigFixed::from(0i32));
    println!("i32 1 {}", BigFixed::from(1i32));
    println!("i32 -1 {}", BigFixed::from(-1i32));
    println!("u32 0 {}", BigFixed::from(0u32));
    println!("u32 1 {}", BigFixed::from(1u32));
    println!("u32 -1 {}", BigFixed::from(-1i32 as u32));
    println!("i64 0 {}", BigFixed::from(0i64));
    println!("i64 1 {}", BigFixed::from(1i64));
    println!("i64 -1 {}", BigFixed::from(-1i64));
    println!("u64 0 {}", BigFixed::from(0u64));
    println!("u64 1 {}", BigFixed::from(1u64));
    println!("u64 -1 {}", BigFixed::from(-1i64 as u64));
    println!("i128 0 {}", BigFixed::from(0i128));
    println!("i128 1 {}", BigFixed::from(1i128));
    println!("i128 -1 {}", BigFixed::from(-1i128));
    println!("u128 0 {}", BigFixed::from(0u128));
    println!("u128 1 {}", BigFixed::from(1u128));
    println!("u128 -1 {}", BigFixed::from(-1i128 as u128));
}
