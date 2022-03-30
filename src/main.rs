use numbers::BigFixed;

pub fn main() {
    println!("i8 0 {}", BigFixed::from(0 as i8));
    println!("i8 1 {}", BigFixed::from(1 as i8));
    println!("i8 -1 {}", BigFixed::from(-1 as i8));
    println!("u8 0 {}", BigFixed::from(0 as u8));
    println!("u8 1 {}", BigFixed::from(1 as u8));
    println!("u8 -1 {}", BigFixed::from(-1 as i8 as u8));
    println!("i16 0 {}", BigFixed::from(0 as i16));
    println!("i16 1 {}", BigFixed::from(1 as i16));
    println!("i16 -1 {}", BigFixed::from(-1 as i16));
    println!("u16 0 {}", BigFixed::from(0 as u16));
    println!("u16 1 {}", BigFixed::from(1 as u16));
    println!("u16 -1 {}", BigFixed::from(-1 as i16 as u16));
    println!("i32 0 {}", BigFixed::from(0 as i32));
    println!("i32 1 {}", BigFixed::from(1 as i32));
    println!("i32 -1 {}", BigFixed::from(-1 as i32));
    println!("u32 0 {}", BigFixed::from(0 as u32));
    println!("u32 1 {}", BigFixed::from(1 as u32));
    println!("u32 -1 {}", BigFixed::from(-1 as i32 as u32));
    println!("i64 0 {}", BigFixed::from(0 as i64));
    println!("i64 1 {}", BigFixed::from(1 as i64));
    println!("i64 -1 {}", BigFixed::from(-1 as i64));
    println!("u64 0 {}", BigFixed::from(0 as u64));
    println!("u64 1 {}", BigFixed::from(1 as u64));
    println!("u64 -1 {}", BigFixed::from(-1 as i64 as u64));
    println!("i128 0 {}", BigFixed::from(0 as i128));
    println!("i128 1 {}", BigFixed::from(1 as i128));
    println!("i128 -1 {}", BigFixed::from(-1 as i128));
    println!("u128 0 {}", BigFixed::from(0 as u128));
    println!("u128 1 {}", BigFixed::from(1 as u128));
    println!("u128 -1 {}", BigFixed::from(-1 as i128 as u128));
}
