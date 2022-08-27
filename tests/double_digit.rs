use bigfixed::*;

use core::ops;

type D1 = u16;
type D2 = u32;
type D3 = u64;
const BITS: usize = 16;
type DD = DoubleDigit<D1>;
type DDD = DoubleDigit<DD>;

#[test]
pub fn multiplication() {
    combined(ops::Mul::mul, DDD::combined_mul);
}

#[test]
pub fn addition() {
    combined(ops::Add::add, DDD::combined_add);
}

#[test]
pub fn bitand() {
    bitwise(ops::BitAnd::bitand, ops::BitAnd::bitand);
}

#[test]
pub fn bitor() {
    bitwise(ops::BitOr::bitor, ops::BitOr::bitor);
}

#[test]
pub fn bitxor() {
    bitwise(ops::BitXor::bitxor, ops::BitXor::bitxor);
}

#[test]
pub fn bitshift() {
    shift();
}

pub fn combined(direct: impl Fn(D3, D3) -> D3, combined_fn: impl Fn(DDD, DDD, &mut DDD, &mut DDD)) {
    let values = [0 as D1, 1 as D1, 2 as D1, 10 as D1, 510 as D1 , 12813 as D1, !(52397 as D1), !(321 as D1), !(12 as D1), !(1 as D1), !(0 as D1)];
    for &m1 in &values {
        for &m2 in &values {
            for &n1 in &values {
                for &n2 in &values {
                    let m1d = m1 as D2;
                    let m2d = m2 as D2;
                    let n1d = n1 as D2;
                    let n2d = n2 as D2;
                    let md = m2d | (m1d << BITS);
                    let nd = n2d | (n1d << BITS);
                    let as_d3 = direct(md as D3, nd as D3);
                    let mdd1 = DoubleDigit::<D1> {
                        greater: m1,
                        lesser: m2,
                    };
                    let ndd1 = DoubleDigit::<D1> {
                        greater: n1,
                        lesser: n2,
                    };
                    let mdd = DoubleDigit::<DoubleDigit<D1>> {
                        greater: DoubleDigit::<D1>::ZERO,
                        lesser: mdd1,
                    };
                    let ndd = DoubleDigit::<DoubleDigit<D1>> {
                        greater: DoubleDigit::<D1>::ZERO,
                        lesser: ndd1,
                    };
                    let mut res = DoubleDigit::<DoubleDigit<D1>>::ZERO;
                    let mut carry = DoubleDigit::<DoubleDigit<D1>>::ZERO;
                    combined_fn(mdd, ndd, &mut res, &mut carry);
                    assert_eq!(as_d3, D3::from(res));
                    assert_eq!(0, D3::from(carry));
                }
            }
        }
    }
}

fn bitwise(direct: impl Fn(D3, D3) -> D3, bitwise_fn: impl Fn(DDD, DDD) -> DDD) {
    let values = [0 as D1, 1 as D1, 2 as D1, 10 as D1, 510 as D1 , 12813 as D1, !(52397 as D1), !(321 as D1), !(12 as D1), !(1 as D1), !(0 as D1)];
    for &m1 in &values {
        for &m2 in &values {
            for &n1 in &values {
                for &n2 in &values {
                    let m1d = m1 as D2;
                    let m2d = m2 as D2;
                    let n1d = n1 as D2;
                    let n2d = n2 as D2;
                    let md = m2d | (m1d << BITS);
                    let nd = n2d | (n1d << BITS);
                    let as_d3 = direct(md as D3, nd as D3);
                    let mdd1 = DoubleDigit::<D1> {
                        greater: m1,
                        lesser: m2,
                    };
                    let ndd1 = DoubleDigit::<D1> {
                        greater: n1,
                        lesser: n2,
                    };
                    let mdd = DoubleDigit::<DoubleDigit<D1>> {
                        greater: DoubleDigit::<D1>::ZERO,
                        lesser: mdd1,
                    };
                    let ndd = DoubleDigit::<DoubleDigit<D1>> {
                        greater: DoubleDigit::<D1>::ZERO,
                        lesser: ndd1,
                    };
                    let res = bitwise_fn(mdd, ndd);
                    assert_eq!(as_d3, D3::from(res));
                }
            }
        }
    }
}

fn shift() {
    let src = 0xA69A69A69D3162B5u128 as D3;
    let srcd = DDD::from(src);
    for i in 0..(4*BITS) {
        assert_eq!(src << i, D3::from(srcd << i));
        assert_eq!(src >> i, D3::from(srcd >> i));
        assert_eq!(src << i, D3::from(srcd << DDD::from(i)));
        assert_eq!(src << i, D3::from(srcd << DDD::from(i)));
    }
}
