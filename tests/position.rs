use bigfixed::*;

type D = u32;

#[test]
fn fix_position() {
    let mut zero_bit = BigFixed::<D> {
        head: 0,
        body: vec![],
        position: Bit(0)
    };
    zero_bit.fix_position().unwrap();
    let zero_pos = BigFixed::<D> {
        head: 0,
        body: vec![],
        position: Position(0)
    };
    assert_eq!(zero_bit, zero_pos, "zero [0] vs zero (0)");
    let mut zero_bit2 = BigFixed::<D> {
        head: 0,
        body: vec![],
        position: Bit(1)
    };
    zero_bit2.fix_position().unwrap();
    assert_eq!(zero_bit2, zero_pos, "zero [1] vs zero (0)");
    let mut one_bit = BigFixed::<D> {
        head: 0,
        body: vec![1],
        position: Bit(0)
    };
    one_bit.fix_position().unwrap();
    let one_pos = BigFixed::<D> {
        head: 0,
        body: vec![1],
        position: Position(0)
    };
    assert_eq!(one_bit, one_pos, "one [0] vs one (0)");
    let mut two_bit = BigFixed::<D> {
        head: 0,
        body: vec![1],
        position: Bit(1)
    };
    two_bit.fix_position().unwrap();
    let two_pos = BigFixed::<D> {
        head: 0,
        body: vec![2],
        position: Position(0)
    };
    assert_eq!(two_bit, two_pos, "one [1] vs two (0)");
    let mut ten_bit = BigFixed::<D> {
        head: 0,
        body: vec![1],
        position: Bit(D::DIGITBITS as isize)
    };
    ten_bit.fix_position().unwrap();
    let ten_pos = BigFixed::<D> {
        head: 0,
        body: vec![1],
        position: Position(1)
    };
    assert_eq!(ten_bit, ten_pos, "one [DIGITBITS] vs one (1)");
    let mut neg_one_bit = BigFixed::<D> {
        head: D::ALLONES,
        body: vec![],
        position: Bit(0)
    };
    neg_one_bit.fix_position().unwrap();
    let neg_one_pos = BigFixed::<D> {
        head: D::ALLONES,
        body: vec![],
        position: Position(0)
    };
    assert_eq!(neg_one_bit, neg_one_pos, "neg one [0] vs neg one (0)");
    let mut neg_three_bit = BigFixed::<D> {
        head: D::ALLONES,
        body: vec![(!3)+1],
        position: Bit(-1)
    };
    neg_three_bit.fix_position().unwrap();
    let neg_three_pos = BigFixed::<D> {
        head: D::ALLONES,
        body: vec![D::GREATESTBIT, !1],
        position: Position(-1)
    };
    assert_eq!(neg_three_bit, neg_three_pos, "negative test");
    let mut neg_two_bit = BigFixed::<D> {
        head: D::ALLONES,
        body: vec![],
        position: Bit(1)
    };
    neg_two_bit.fix_position().unwrap();
    let neg_two_pos = BigFixed::<D> {
        head: D::ALLONES,
        body: vec![D::ALLONES << 1],
        position: Position(0)
    };
    assert_eq!(neg_two_bit, neg_two_pos, "negative empty body");
}

#[test]
fn format() {
    let real_zero = BigFixed {
        head: D::ZERO,
        body: vec![],
        position: Position(0)
    };
    assert_eq!(BigFixed::<D>::construct(D::ZERO, vec![], Position(0)).unwrap(), real_zero, "zero zero");
    assert_eq!(BigFixed::<D>::construct(D::ZERO, vec![0; 10], Bit(-140)).unwrap(), real_zero, "zero zeroes");
    let real_neg_one = BigFixed {
        head: D::ALLONES,
        body: vec![],
        position: Position(0)
    };
    assert_eq!(BigFixed::<D>::construct(D::ALLONES, vec![], Bit(0)).unwrap(), real_neg_one, "neg one empty");
    assert_eq!(BigFixed::<D>::construct(D::ALLONES, vec![D::ALLONES; 2], Position(0)).unwrap(), real_neg_one, "neg one heads");
    let real_t1 = BigFixed {
        head: D::ALLONES,
        body: vec![5],
        position: Position(1)
    };
    assert_eq!(BigFixed::<D>::construct(D::ALLONES, vec![0, 0, 5, D::ALLONES], Bit(D::DIGITBITS as isize * -1)).unwrap(), real_t1, "negative with body both sides");
    println!("all good");
}