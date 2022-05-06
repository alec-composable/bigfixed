use bigfixed::{digit::*, Index, BigFixed};

#[test]
fn add_digit() {
    let mut x = BigFixed {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    };
    x.add_digit(ALLONES, Index::Position(0)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![ALLONES],
        position: Index::Position(0)
    }).unwrap(), "0 + 9");
    x.add_digit(1, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![1, 0, ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9 + 0.01");
    x.add_digit(ALLONES, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![1, ALLONES, ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9.01 + 0.9");
    x.add_digit(ALLONES, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![0, 0, 0, 1],
        position: Index::Position(-2)
    }).unwrap(), "9.91 + 0.09");
    x = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Index::Position(1)
    };
    x.add_digit(1, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixed {
        head: ALLONES,
        body: vec![1, 0],
        position: Index::Position(-1)
    }).unwrap(), "-10 + 0.1");
    x.add_digit(1, Index::Position(1)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![1, 0, 0],
        position: Index::Position(-1)
    }).unwrap(), "-9.99 + 10");
}

#[test]
fn add_digit_drop_overflow() {
    let mut x = BigFixed {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    };
    x.add_digit_drop_overflow(ALLONES, Index::Position(0)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![],
        position: Index::Position(0)
    }).unwrap(), "0 + 9");
    x[0] = ALLONES; // the overflow which was dropped
    x.add_digit_drop_overflow(1, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![1, 0, ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9 + 0.01");
    x.add_digit_drop_overflow(ALLONES, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![1, ALLONES, ALLONES],
        position: Index::Position(-2)
    }).unwrap(), "9.01 + 0.9");
    x.add_digit_drop_overflow(ALLONES, Index::Position(-2)).ok();
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![0, 0, 0],
        position: Index::Position(-2)
    }).unwrap(), "9.91 + 0.09");
    x = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Index::Position(1)
    };
    x.add_digit_drop_overflow(1, Index::Position(-1)).ok();
    assert!(x.full_eq(&BigFixed {
        head: ALLONES,
        body: vec![1, 0],
        position: Index::Position(-1)
    }).unwrap(), "-10 + 0.1");
    x.add_digit_drop_overflow(1, Index::Position(1)).ok();
    assert!(x.full_eq(&BigFixed {
        head: ALLONES,
        body: vec![1, 0],
        position: Index::Position(-1)
    }).unwrap(), "-9.99 + 10");
}