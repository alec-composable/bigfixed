use bigfixed::{digit::*, Index::*, BigFixed};

fn check<E>(x: Result<bool, E>, expected: bool, message: &str) {
    match x {
        Ok(x) if x == expected => {},
        Ok(_) => panic!("test failed: {}", message),
        Err(_) => panic!("internal error: {}", message)
    }
}

#[test]
fn fix_position() {
    let mut zero_bit = BigFixed {
        head: 0,
        body: vec![],
        position: Bit(0)
    };
    check(zero_bit.fix_position(), true, "fixing position");
    let zero_pos = BigFixed {
        head: 0,
        body: vec![],
        position: Position(0)
    };
    check(zero_bit.full_eq(&zero_pos), true, "zero [0] vs zero (0)");
    let mut zero_bit2 = BigFixed {
        head: 0,
        body: vec![],
        position: Bit(1)
    };
    check(zero_bit2.fix_position(), true, "fixing position");
    check(zero_bit2.full_eq(&zero_pos), true, "zero [1] vs zero (0)");
    let mut one_bit = BigFixed {
        head: 0,
        body: vec![1],
        position: Bit(0)
    };
    check(one_bit.fix_position(), true, "fixing position");
    let one_pos = BigFixed {
        head: 0,
        body: vec![1],
        position: Position(0)
    };
    check(one_bit.full_eq(&one_pos), true, "one [0] vs one (0)");
    let mut two_bit = BigFixed {
        head: 0,
        body: vec![1],
        position: Bit(1)
    };
    check(two_bit.fix_position(), true, "fixing position");
    let two_pos = BigFixed {
        head: 0,
        body: vec![2],
        position: Position(0)
    };
    check(two_bit.full_eq(&two_pos), true, "one [1] vs two (0)");
    let mut ten_bit = BigFixed {
        head: 0,
        body: vec![1],
        position: Bit(DIGITBITS as isize)
    };
    check(ten_bit.fix_position(), true, "fixing position");
    let ten_pos = BigFixed {
        head: 0,
        body: vec![1],
        position: Position(1)
    };
    check(ten_bit.full_eq(&ten_pos), true, "one [DIGITBITS] vs one (1)");
    let mut neg_one_bit = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Bit(0)
    };
    check(neg_one_bit.fix_position(), true, "fixing position");
    let neg_one_pos = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Position(0)
    };
    check(neg_one_bit.full_eq(&neg_one_pos), true, "neg one [0] vs neg one (0)");
    let mut neg_three_bit = BigFixed {
        head: ALLONES,
        body: vec![3],
        position: Bit(-1)
    };
    check(neg_three_bit.fix_position(), true, "fixing position");
    let neg_three_pos = BigFixed {
        head: ALLONES,
        body: vec![GREATESTBIT, GREATESTBIT | 1],
        position: Position(-1)
    };
    check(neg_three_bit.full_eq(&neg_three_pos), true, "negative test");
    let mut neg_two_bit = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Bit(1)
    };
    check(neg_two_bit.fix_position(), true, "fixing position");
    let neg_two_pos = BigFixed {
        head: ALLONES,
        body: vec![ALLONES << 1],
        position: Position(0)
    };
    check(neg_two_bit.full_eq(&neg_two_pos), true, "negative empty body");
}

#[test]
fn format() {
    let real_zero = BigFixed {
        head: 0,
        body: vec![],
        position: Position(0)
    };
    assert_eq!(BigFixed::construct(0, vec![], Position(0)).unwrap(), real_zero, "zero zero");
    assert_eq!(BigFixed::construct(0, vec![0,0,0,0,0,0,0,0,0,0], Bit(-140)).unwrap(), real_zero, "zero zeroes");
    let real_neg_one = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Position(0)
    };
    assert_eq!(BigFixed::construct(ALLONES, vec![], Bit(0)).unwrap(), real_neg_one, "neg one empty");
    assert_eq!(BigFixed::construct(ALLONES, vec![ALLONES, ALLONES], Position(0)).unwrap(), real_neg_one, "neg one heads");
    let real_t1 = BigFixed {
        head: ALLONES,
        body: vec![5],
        position: Position(1)
    };
    assert_eq!(BigFixed::construct(ALLONES, vec![0,0,5,ALLONES], Bit(DIGITBITS as isize * -1)).unwrap(), real_t1, "negative with body both sides");
}

// format_c

#[test]
fn ensure_valid_range() {
    let padded = BigFixed {
        head: ALLONES,
        body: vec![0, 0, 6, ALLONES, ALLONES],
        position: Position(-6)
    };
    let mut no_growth = padded.clone();
    check(no_growth.ensure_valid_range(Position(0), Position(0)), false, "ensuring valid range");
    assert_eq!(padded, no_growth, "no growth internal");
    check(no_growth.ensure_valid_range(no_growth.position, (no_growth.position + 1isize).unwrap()), false, "ensuring valid range");
    assert_eq!(padded, no_growth, "no growth start");
    check(no_growth.ensure_valid_range((no_growth.body_high().unwrap() - 1isize).unwrap(), no_growth.body_high().unwrap()), false, "ensuring valid range");
    assert_eq!(padded, no_growth, "no growth end");
    check(no_growth.ensure_valid_range(no_growth.position, no_growth.body_high().unwrap()), false, "ensuring valid range");
    assert_eq!(padded, no_growth, "no growth full");
    let mut growth = BigFixed {
        head: ALLONES,
        body: vec![6],
        position: Position(-4)
    };
    check(growth.ensure_valid_range(Bit(-5*(DIGITBITS as isize) - 1), Position(-3)), true, "ensuring valid range");
    assert_eq!(growth, BigFixed {
        head: ALLONES,
        body: vec![0, 0, 6],
        position: Position(-6)
    }, "bit growth below");
    check(growth.ensure_valid_range(Bit(-3*(DIGITBITS as isize) - 3), Position(-2)), true, "ensuring valid range");
    assert_eq!(growth, BigFixed {
        head: ALLONES,
        body: vec![0, 0, 6, ALLONES],
        position: Position(-6)
    }, "bit growth above");
    check(growth.ensure_valid_range(Position(-6), Position(-1)), true, "ensuring valid range");
    assert_eq!(growth, padded, "position growth both ends");
    let mut x = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Position(1)
    };
    check(x.ensure_valid_position(Position(-1)), true, "ensuring valid range");
    assert_eq!(x, BigFixed {
        head: ALLONES,
        body: vec![0, 0],
        position: Position(-1)
    }, "negative no body to body below");
    x = BigFixed {
        head: ALLONES,
        body: vec![],
        position: Position(-1)
    };
    check(x.ensure_valid_position(Position(1)), true, "ensuring valid position");
    assert_eq!(x, BigFixed {
        head: ALLONES,
        body: vec![ALLONES, ALLONES, ALLONES],
        position: Position(-1)
    }, "negative no body to body above");
}

// int
// frac

#[test]
fn shift() {
    assert_eq!(
        BigFixed {
            head: 0,
            body: vec![],
            position: Bit(100)
        }.shift(Bit(0)).unwrap(),
        BigFixed {
            head: 0,
            body: vec![],
            position: Position(0)
        },
        "zero shift"
    );
    assert_eq!(
        BigFixed {
            head: 0,
            body: vec![2],
            position: Position(0)
        }.shift(Bit(-1)).unwrap(),
        BigFixed {
            head: 0,
            body: vec![1],
            position: Position(0)
        },
        "2 >> 1 == 1"
    );
    assert_eq!(
        BigFixed {
            head: 0,
            body: vec![1],
            position: Position(0)
        }.shift(Bit(1)).unwrap(),
        BigFixed {
            head: 0,
            body: vec![2],
            position: Position(0)
        },
        "1 << 1 == 2"
    );
    assert_eq!(
        BigFixed {
            head: ALLONES,
            body: vec![],
            position: Position(0)
        }.shift(Position(100)).unwrap(),
        BigFixed {
            head: ALLONES,
            body: vec![],
            position: Position(100)
        },
        "-1 shifting left 100 positions"
    );
    assert_eq!(
        BigFixed {
            head: ALLONES,
            body: vec![1, ALLONES, 0, 3],
            position: Bit(4)
        }.shift(Bit(-2)).unwrap(),
        BigFixed {
            head: ALLONES,
            body: vec![1 << 2, ALLONES << 2, 3, 3 << 2, ALLONES << 2],
            position: Position(0)
        },
        "negative multinumber 1"
    );
}

// cutoff position
// greatest bit position
