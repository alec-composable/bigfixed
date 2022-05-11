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

#[test]
fn negate() {
    for i in -3..=3 {
        assert_eq!((-BigFixed::from(i)).unwrap(), BigFixed::from(-i), "{}", i);
        let shift = Index::Position(if i % 2 == 0 {
            i
        } else {
            -i
        });
        assert_eq!((-BigFixed::from(i).shift(shift).unwrap()).unwrap(), BigFixed::from(-i).shift(shift).unwrap(), "{}", i);
    }
    assert_eq!(-(
        BigFixed::construct(0, vec![53, 128, (ALLONES << 8) | 42], Index::Bit(-190))
    ).unwrap(),
        BigFixed::construct(ALLONES, vec![!53 + 1, !128, 255 & !42], Index::Bit(-190)),
     "big number");
}

#[test]
fn abs() {
    for i in -10..=10 {
        assert_eq!(
            BigFixed::from(i).shift(Index::Bit(15*i)).unwrap().abs().unwrap(),
            BigFixed::from(if i < 0 {-i} else {i}).shift(Index::Bit(15*i)).unwrap(),
            "{}", i
        );
    }
}

#[test]
fn add() {
    let big = 0xE2103A85FD47AB2E94F2E5108CB5E24i128;
    for i in 0..=12 {
        let a = (big >> (10*i + 4)) as i128;
        for j in 0..3 {
            let b = (a << j) * if (j*i) / 2 == 0 {1} else {-1};
            assert_eq!(BigFixed::from(a + b), BigFixed::from(a) + BigFixed::from(b), "{} {}", i, j);
        }
    }
    assert_eq!(
        BigFixed::construct(0, vec![ALLONES, ALLONES, ALLONES, ALLONES, ALLONES], Index::Position(-4)).unwrap()
            + BigFixed::construct(0, vec![1], Index::Position(-3)).unwrap(),
        BigFixed::construct(0, vec![ALLONES, 0, 0, 0, 0, 1], Index::Position(-4)).unwrap(),
        "9.9999 + 0.001"
    );
    assert_eq!(
        BigFixed::from(1).shift(Index::Position(5)).unwrap() + BigFixed::from(0),
        BigFixed::from(1).shift(Index::Position(5)).unwrap(),
        "10000 + 0"
    );
}

#[test]
fn bitwise() {
    assert_eq!(
        BigFixed::construct(0, vec![1, 255, 9, ALLONES, !1], Index::Position(7)).unwrap()
         & BigFixed::construct(0, vec![100, 5, 28], Index::Position(8)).unwrap(),
        BigFixed::construct(0, vec![255 & 100, 9 & 5, 28], Index::Position(8)).unwrap()
    );
    assert_eq!(
        BigFixed::construct(0, vec![1, 255, 9, ALLONES, !1], Index::Position(7)).unwrap()
         | BigFixed::construct(0, vec![100, 5, 28], Index::Position(8)).unwrap(),
        BigFixed::construct(0, vec![1, 255 | 100, 9 | 5, ALLONES, !1], Index::Position(7)).unwrap()
    );
    assert_eq!(
        BigFixed::construct(0, vec![1, 255, 9, ALLONES, !1], Index::Position(7)).unwrap()
         ^ BigFixed::construct(0, vec![100, 5, 28], Index::Position(8)).unwrap(),
        BigFixed::construct(0, vec![1, 255 ^ 100, 9 ^ 5, !28, !1], Index::Position(7)).unwrap()
    );
    assert_eq!(
        (!BigFixed::construct(0, vec![1, 255, 9, ALLONES, !1], Index::Position(7)).unwrap()).unwrap(),
        BigFixed::construct(ALLONES, vec![!1 + 1, !255, !9, 0, 1], Index::Position(7)).unwrap()
    );
}

#[test]
fn mul() {
    let big = 0xE2103A85FD47AB2i128;
    for i in 0..=5 {
        let a = (big >> (10*i + 4)) as i128;
        for j in 0..3 {
            let b = (a << j) * if (j*i) / 2 == 0 {1} else {-1};
            assert_eq!(BigFixed::from(a * b), BigFixed::from(a) * BigFixed::from(b), "{} {}", i, j);
        }
    }
    assert_eq!(
        BigFixed::construct(0, vec![ALLONES, ALLONES, ALLONES], Index::Position(-4)).unwrap()
            * BigFixed::construct(ALLONES, vec![!1 + 1, !1], Index::Position(1)).unwrap(),
        BigFixed::construct(ALLONES, vec![1, 1, 0, ALLONES, !1], Index::Position(-3)).unwrap(),
        "0.0999 * -110 == -10.989"
    )
}

#[test]
fn div() {
    //let top = BigFixed::from(1).shift(Index::Position(5));
    //let bottom = BigFixed::from(17).shift(Index::Position(-3));

}
