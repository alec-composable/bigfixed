/*use bigfixed::{digit::*, Index, BigFixed};

use std::{iter};

fn test(template: &BigFixed, tester: &BigFixed, msg: &str) {
    match template.full_eq(tester) {
        Ok(b) => assert!(b, "{}", msg),
        Err(_) => panic!("internal failure testing {}", msg)
    }
}

#[test]
fn from_ints() {
    let zero = BigFixed {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    };
    test(&zero, &BigFixed::from(0u8), "u8");
    test(&zero, &BigFixed::from(0i8), "i8");
    test(&zero, &BigFixed::from(0u16), "u16");
    test(&zero, &BigFixed::from(0i16), "i16");
    test(&zero, &BigFixed::from(0u32), "u32");
    test(&zero, &BigFixed::from(0i32), "i32");
    test(&zero, &BigFixed::from(0u64), "u64");
    test(&zero, &BigFixed::from(0i64), "i64");
    test(&zero, &BigFixed::from(0u128), "u128");
    test(&zero, &BigFixed::from(0i128), "i128");
    
    let one = BigFixed {
        head: 0,
        body: vec![1],
        position: Index::Position(0)
    };
    test(&one, &BigFixed::from(1u8), "u8");
    test(&one, &BigFixed::from(1i8), "i8");
    test(&one, &BigFixed::from(1u16), "u16");
    test(&one, &BigFixed::from(1i16), "i16");
    test(&one, &BigFixed::from(1u32), "u32");
    test(&one, &BigFixed::from(1i32), "i32");
    test(&one, &BigFixed::from(1u64), "u64");
    test(&one, &BigFixed::from(1i64), "i64");
    test(&one, &BigFixed::from(1u128), "u128");
    test(&one, &BigFixed::from(1i128), "i128");
    
    let neg_one = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Index::Position(0)
    };
    test(&neg_one, &BigFixed::from(-1i8), "i8");
    test(&neg_one, &BigFixed::from(-1i16), "i16");
    test(&neg_one, &BigFixed::from(-1i32), "i32");
    test(&neg_one, &BigFixed::from(-1i64), "i64");
    test(&neg_one, &BigFixed::from(-1i128), "i128");
    
    let lots_of_ones = BigFixed {
        head: 0,
        body: iter::repeat(ALLONES).take(128 / DIGITBITS).collect(),
        position: Index::Position(0)
    };
    test(&lots_of_ones, &BigFixed::from(-1i128 as u128), "-1u128");
}

#[test]
fn to_ints() {
    // signed ints are waiting on ops...
    let zero = BigFixed::from(0u8);
    assert_eq!(0u8, u8::from(&zero), "u8");
    //assert_eq!(0i8, i8::from(&zero), "i8");
    assert_eq!(0u16, u16::from(&zero), "u16");
    //assert_eq!(0i16, i16::from(&zero), "i16");
    assert_eq!(0u32, u32::from(&zero), "u32");
    //assert_eq!(0i32, i32::from(&zero), "i32");
    assert_eq!(0u64, u64::from(&zero), "u64");
    //assert_eq!(0i64, i64::from(&zero), "i64");
    assert_eq!(0u128, u128::from(&zero), "u128");
    //assert_eq!(0i128, i128::from(&zero), "i128");

    let neg_one = BigFixed::from(-1i8);
    assert_eq!(-1i8 as u8, u8::from(&neg_one), "-1u8");
    assert_eq!(-1i16 as u16, u16::from(&neg_one), "-1u16");
    assert_eq!(-1i32 as u32, u32::from(&neg_one), "-1u32");
    assert_eq!(-1i64 as u64, u64::from(&neg_one), "-1u64");
    assert_eq!(-1i128 as u128, u128::from(&neg_one), "-1u128");
}

#[test]
fn floats() {
    for a in [0f32, 1f32, 10f32] {
        for b in [1f32, 2f32, 100001f32] {
            let c = a / b;
            assert_eq!(c, f32::from(BigFixed::from(c)), "f32 {}", c);
        }
    }
    for a in [0f64, 1f64, 10f64] {
        for b in [1f64, 2f64, 100001f64] {
            let c = a / b;
            assert_eq!(c, f64::from(BigFixed::from(c)), "f64 {}", c);
        }
    }
}*/
