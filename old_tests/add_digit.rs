use bigfixed::*;

type D = u32;

#[test]
fn add_digit() {
    let mut x = BigFixedVec::<D> {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    };
    x.add_digit(D::ALLONES, Index::Position(0)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![D::ALLONES],
        position: Index::Position(0)
    }).unwrap(), "0 + 9");
    x.add_digit(1, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![1, 0, D::ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9 + 0.01");
    x.add_digit(D::ALLONES, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![1, D::ALLONES, D::ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9.01 + 0.9");
    x.add_digit(D::ALLONES, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![0, 0, 0, 1],
        position: Index::Position(-2)
    }).unwrap(), "9.91 + 0.09");
    x = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Index::Position(1)
    };
    x.add_digit(1, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: D::ALLONES,
        body: vec![1, 0],
        position: Index::Position(-1)
    }).unwrap(), "-10 + 0.1");
    x.add_digit(1, Index::Position(1)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![1, 0, 0],
        position: Index::Position(-1)
    }).unwrap(), "-9.99 + 10");
}

#[test]
fn add_digit_drop_overflow() {
    let mut x = BigFixedVec::<D> {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    };
    x.add_digit_drop_overflow(D::ALLONES, Index::Position(0)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    }).unwrap(), "0 + 9");
    x[Index::Position(0)] = D::ALLONES; // the overflow which was dropped
    x.add_digit_drop_overflow(1, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![1, 0, D::ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9 + 0.01");
    x.add_digit_drop_overflow(D::ALLONES, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![1, D::ALLONES, D::ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9.01 + 0.9");
    x.add_digit_drop_overflow(D::ALLONES, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: 0,
        body: vec![0, 0, 0],
        position: Index::Position(-2)
    }).unwrap(), "9.91 + 0.09");
    x = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Index::Position(1)
    };
    x.add_digit_drop_overflow(1, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: D::ALLONES,
        body: vec![1, 0],
        position: Index::Position(-1)
    }).unwrap(), "-10 + 0.1");
    x.add_digit_drop_overflow(1, Index::Position(1)).ok();
    assert!(x.full_eq(&BigFixedVec {
        head: D::ALLONES,
        body: vec![1, 0],
        position: Index::Position(-1)
    }).unwrap(), "-9.99 + 10");
}