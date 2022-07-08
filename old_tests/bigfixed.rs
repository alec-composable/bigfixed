/*use bigfixed::{digit::*, index::*, cutoff::*, BigFixed, BigFixedVec};

fn check<E>(x: Result<bool, E>, expected: bool, message: &str) {
    match x {
        Ok(x) if x == expected => {},
        Ok(_) => panic!("test failed: {}", message),
        Err(_) => panic!("internal error: {}", message)
    }
}

type D = Digit16;

#[test]
fn fix_position() {
    let mut zero_bit = BigFixedVec {
        head: D::ZERO,
        body: vec![],
        position: Bit(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(zero_bit.fix_position(), true, "fixing position");
    let zero_pos = BigFixedVec {
        head: D::ZERO,
        body: vec![],
        position: Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(zero_bit.full_eq(&zero_pos), true, "zero [0] vs zero (0)");
    let mut zero_bit2 = BigFixedVec {
        head: D::ZERO,
        body: vec![],
        position: Bit(1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(zero_bit2.fix_position(), true, "fixing position");
    check(zero_bit2.full_eq(&zero_pos), true, "zero [1] vs zero (0)");
    let mut one_bit = BigFixedVec {
        head: D::ZERO,
        body: vec![D::ONE],
        position: Bit(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(one_bit.fix_position(), true, "fixing position");
    let one_pos = BigFixedVec {
        head: D::ZERO,
        body: vec![D::ONE],
        position: Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(one_bit.full_eq(&one_pos), true, "one [0] vs one (0)");
    let mut two_bit = BigFixedVec {
        head: D::ZERO,
        body: vec![D::ONE],
        position: Bit(1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(two_bit.fix_position(), true, "fixing position");
    let two_pos = BigFixedVec {
        head: D::ZERO,
        body: vec![ D {value: 2}],
        position: Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(two_bit.full_eq(&two_pos), true, "one [1] vs two (0)");
    let mut ten_bit = BigFixedVec {
        head: D::ZERO,
        body: vec![D::ONE],
        position: Bit(D::DIGITBITS as isize),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(ten_bit.fix_position(), true, "fixing position");
    let ten_pos = BigFixedVec {
        head: D::ZERO,
        body: vec![D::ONE],
        position: Position(1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(ten_bit.full_eq(&ten_pos), true, "one [DIGITBITS] vs one (1)");
    let mut neg_one_bit = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Bit(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(neg_one_bit.fix_position(), true, "fixing position");
    let neg_one_pos = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(neg_one_bit.full_eq(&neg_one_pos), true, "neg one [0] vs neg one (0)");
    let mut neg_three_bit = BigFixedVec {
        head: D::ALLONES,
        body: vec![ D {value: 2} + D::ONE],
        position: Bit(-1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(neg_three_bit.fix_position(), true, "fixing position");
    let neg_three_pos = BigFixedVec {
        head: D::ALLONES,
        body: vec![D::GREATESTBIT, D::GREATESTBIT | D::ONE],
        position: Position(-1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(neg_three_bit.full_eq(&neg_three_pos), true, "negative test");
    let mut neg_two_bit = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Bit(1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(neg_two_bit.fix_position(), true, "fixing position");
    let neg_two_pos = BigFixedVec {
        head: D::ALLONES,
        body: vec![D::ALLONES << D::ONE],
        position: Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(neg_two_bit.full_eq(&neg_two_pos), true, "negative empty body");
}

#[test]
fn format() {
    let real_zero = BigFixedVec {
        head: D::ZERO,
        body: vec![],
        position: Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    assert_eq!(BigFixedVec::<D>::construct(D::ZERO, vec![], Position(0)).unwrap(), real_zero, "zero zero");
    assert_eq!(BigFixedVec::<D>::construct(D::ZERO, vec![D::ZERO; 10], Bit(-140)).unwrap(), real_zero, "zero zeroes");
    let real_neg_one = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Position(0),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    assert_eq!(BigFixedVec::<D>::construct(D::ALLONES, vec![], Bit(0)).unwrap(), real_neg_one, "neg one empty");
    assert_eq!(BigFixedVec::<D>::construct(D::ALLONES, vec![D::ALLONES; 2], Position(0)).unwrap(), real_neg_one, "neg one heads");
    let real_t1 = BigFixedVec {
        head: D::ALLONES,
        body: vec![D {value: 5}],
        position: Position(1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    assert_eq!(BigFixedVec::<D>::construct(D::ALLONES, vec![D::ZERO, D::ZERO, D {value: 5}, D::ALLONES], Bit(D::DIGITBITS as isize * -1)).unwrap(), real_t1, "negative with body both sides");
}

// format_c

#[test]
fn ensure_valid_range() {
    let padded = BigFixedVec {
        head: D::ALLONES,
        body: vec![D::ZERO, D::ZERO, D {value: 6}, D::ALLONES, D::ALLONES],
        position: Position(-6),
        zero_copy: D::ZERO,
        one_copy: D::ONE
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
    let mut growth = BigFixedVec {
        head: D::ALLONES,
        body: vec![D {value: 6}],
        position: Position(-4),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(growth.ensure_valid_range(Bit(-5*(D::DIGITBITS as isize) - 1), Position(-3)), true, "ensuring valid range");
    assert_eq!(growth, BigFixedVec {
        head: D::ALLONES,
        body: vec![D::ZERO, D::ZERO, D {value: 6}],
        position: Position(-6),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    }, "bit growth below");
    check(growth.ensure_valid_range(Bit(-3*(D::DIGITBITS as isize) - 3), Position(-2)), true, "ensuring valid range");
    assert_eq!(growth, BigFixedVec {
        head: D::ALLONES,
        body: vec![D::ZERO, D::ZERO, D {value: 6}, D::ALLONES],
        position: Position(-6),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    }, "bit growth above");
    check(growth.ensure_valid_range(Position(-6), Position(-1)), true, "ensuring valid range");
    assert_eq!(growth, padded, "position growth both ends");
    let mut x = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Position(1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(x.ensure_valid_position(Position(-1)), true, "ensuring valid range");
    assert_eq!(x, BigFixedVec {
        head: D::ALLONES,
        body: vec![D::ZERO, D::ZERO],
        position: Position(-1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    }, "negative no body to body below");
    x = BigFixedVec {
        head: D::ALLONES,
        body: vec![],
        position: Position(-1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    };
    check(x.ensure_valid_position(Position(1)), true, "ensuring valid position");
    assert_eq!(x, BigFixedVec {
        head: D::ALLONES,
        body: vec![D::ALLONES, D::ALLONES, D::ALLONES],
        position: Position(-1),
        zero_copy: D::ZERO,
        one_copy: D::ONE
    }, "negative no body to body above");
}

// int
// frac

#[test]
fn shift() {
    assert_eq!(
        BigFixedVec {
            head: D::ZERO,
            body: vec![],
            position: Bit(100),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        }.shift(Bit(0)).unwrap(),
        BigFixedVec {
            head: D::ZERO,
            body: vec![],
            position: Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        },
        "zero shift"
    );
    assert_eq!(
        BigFixedVec {
            head: D::ZERO,
            body: vec![ D {value: 2}],
            position: Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        }.shift(Bit(-1)).unwrap(),
        BigFixedVec {
            head: D::ZERO,
            body: vec![D::ONE],
            position: Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        },
        "2 >> 1 == 1"
    );
    assert_eq!(
        BigFixedVec {
            head: D::ZERO,
            body: vec![D::ONE],
            position: Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        }.shift(Bit(1)).unwrap(),
        BigFixedVec {
            head: D::ZERO,
            body: vec![ D {value: 2}],
            position: Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        },
        "1 << 1 == 2"
    );
    assert_eq!(
        BigFixedVec {
            head: D::ALLONES,
            body: vec![],
            position: Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        }.shift(Position(100)).unwrap(),
        BigFixedVec {
            head: D::ALLONES,
            body: vec![],
            position: Position(100),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        },
        "-1 shifting left 100 positions"
    );
    assert_eq!(
        BigFixedVec {
            head: D::ALLONES,
            body: vec![D::ONE, D::ALLONES, D::ZERO, D {value: 3}],
            position: Bit(4),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        }.shift(Bit(-2)).unwrap(),
        BigFixedVec {
            head: D::ALLONES,
            body: vec![D::ONE << 2usize, D::ALLONES << 2usize, D {value: 3}, D {value: 3} << 2usize, D::ALLONES << 2usize],
            position: Position(0),
            zero_copy: D::ZERO,
            one_copy: D::ONE
        },
        "negative multinumber 1"
    );
}

// cutoff position
// greatest bit position

#[test]
fn cutoff() {
    macro_rules! test_cutoff_fixed_bit {
        ($src: expr, $bit: expr, $round: ident, $head: expr, $body: expr, $position: expr, $msg: expr) => {
            let mut src = BigFixedVec::<D>::from($src);
            src.cutoff(Cutoff {
                fixed: Some(Index::Bit($bit)),
                floating: None,
                round: Rounding::$round
            }).unwrap();
            assert_eq!(
                src,
                BigFixedVec::<D> {
                    head: $head,
                    body: $body,
                    position: Position($position),
                    zero_copy: D::ZERO,
                    one_copy: D::ONE
                },
                $msg
            );
        };
    }
    macro_rules! test_cutoff_floating_bit {
        ($src: expr, $bit: expr, $round: ident, $head: expr, $body: expr, $position: expr, $msg: expr) => {
            let mut src = BigFixedVec::<D>::from($src);
            src.cutoff(Cutoff {
                fixed: None,
                floating: Some(Index::Bit($bit)),
                round: Rounding::$round
            }).unwrap();
            assert_eq!(
                src,
                BigFixedVec::<D> {
                    head: $head,
                    body: $body,
                    position: Position($position),
                    zero_copy: D::ZERO,
                    one_copy: D::ONE
                },
                $msg
            );
        };
    }
    macro_rules! test_cutoff_fixed_position {
        ($src: expr, $pos: expr, $round: ident, $head: expr, $body: expr, $position: expr, $msg: expr) => {
            let mut src = BigFixedVec::<D>::from($src);src.cutoff(Cutoff {
                fixed: Some(Index::Position($pos)),
                floating: None,
                round: Rounding::$round
            }).unwrap();
            assert_eq!(
                src,
                BigFixedVec::<D> {
                    head: $head,
                    body: $body,
                    position: Position($position),
                    zero_copy: D::ZERO,
                    one_copy: D::ONE
                },
                $msg
            );
        };
    }
    macro_rules! test_cutoff_floating_position {
        ($src: expr, $pos: expr, $round: ident, $head: expr, $body: expr, $position: expr, $msg: expr) => {
            let mut src = BigFixedVec::<D>::from($src);src.cutoff(Cutoff {
                fixed: None,
                floating: Some(Index::Position($pos)),
                round: Rounding::$round
            }).unwrap();
            assert_eq!(
                src,
                BigFixedVec::<D> {
                    head: $head,
                    body: $body,
                    position: Position($position),
                    zero_copy: D::ZERO,
                    one_copy: D::ONE
                },
                $msg
            );
        };
    }
    // msg format: "src fixed floating"
    test_cutoff_fixed_bit!(0, 0, Floor, D::ZERO, vec![], 0, "0 [0] .");
    test_cutoff_floating_bit!(0, 0, Floor, D::ZERO, vec![], 0, "0 . [0]");
    test_cutoff_fixed_position!(0, 0, Floor, D::ZERO, vec![], 0, "0 (0) .");
    test_cutoff_floating_position!(0, 0, Floor, D::ZERO, vec![], 0, "0 . (0)");
    test_cutoff_fixed_bit!(0, 6, Floor, D::ZERO, vec![], 0, "0 [6] .");
    test_cutoff_floating_bit!(0, 6, Floor, D::ZERO, vec![], 0, "0 . [6]");
    test_cutoff_fixed_position!(0, 6, Floor, D::ZERO, vec![], 0, "0 (6) .");
    test_cutoff_floating_position!(0, 6, Floor, D::ZERO, vec![], 0, "0 . (6)");
    test_cutoff_fixed_bit!(0, -6, Floor, D::ZERO, vec![], 0, "0 [-6] .");
    test_cutoff_floating_bit!(0, -6, Floor, D::ZERO, vec![], 0, "0 . [-6]");
    test_cutoff_fixed_position!(0, -6, Floor, D::ZERO, vec![], 0, "0 (-6) .");
    test_cutoff_floating_position!(0, -6, Floor, D::ZERO, vec![], 0, "0 . (-6)");

    test_cutoff_fixed_bit!(1, 0, Floor, D::ZERO, vec![D::ONE], 0, "1 [0] .");
    test_cutoff_floating_bit!(1, 0, Floor, D::ZERO, vec![D::ONE], 0, "1 . [0]");
    test_cutoff_fixed_position!(1, 0, Floor, D::ZERO, vec![D::ONE], 0, "1 (0) .");
    test_cutoff_floating_position!(1, 0, Floor, D::ZERO, vec![D::ONE], 0, "1 . (0)");
    test_cutoff_fixed_bit!(1, 6, Floor, D::ZERO, vec![], 0, "1 [6] .");
    test_cutoff_floating_bit!(1, 6, Floor, D::ZERO, vec![D::ONE], 0, "1 . [6]");
    test_cutoff_fixed_position!(1, 6, Floor, D::ZERO, vec![], 0, "1 (6) .");
    test_cutoff_floating_position!(1, 6, Floor, D::ZERO, vec![D::ONE], 0, "1 . (6)");
    test_cutoff_fixed_bit!(1, -6, Floor, D::ZERO, vec![D::ONE], 0, "1 [-6] .");
    test_cutoff_floating_bit!(1, -6, Floor, D::ZERO, vec![D::ONE], 0, "1 . [-6]");
    test_cutoff_fixed_position!(1, -6, Floor, D::ZERO, vec![D::ONE], 0, "1 (-6) .");
    test_cutoff_floating_position!(1, -6, Floor, D::ZERO, vec![D::ONE], 0, "1 . (-6)");

    test_cutoff_fixed_bit!(-1, 0, Floor, D::ALLONES, vec![], 0, "-1 [0] .");
    test_cutoff_floating_bit!(-1, 0, Floor, D::ALLONES, vec![], 0, "-1 . [0]");
    test_cutoff_fixed_position!(-1, 0, Floor, D::ALLONES, vec![], 0, "-1 (0) .");
    test_cutoff_floating_position!(-1, 0, Floor, D::ALLONES, vec![], 0, "-1 . (0)");
    test_cutoff_fixed_bit!(-1, 6, Floor, D::ALLONES, vec![D::ALLONES << 6usize], 0, "-1 [6] .");
    test_cutoff_floating_bit!(-1, 6, Floor, D::ALLONES, vec![], 0, "-1 . [6]");
    test_cutoff_fixed_position!(-1, 6, Floor, D::ALLONES, vec![], 6, "-1 (6) .");
    test_cutoff_floating_position!(-1, 6, Floor, D::ALLONES, vec![], 0, "-1 . (6)");
    test_cutoff_fixed_bit!(-1, -6, Floor, D::ALLONES, vec![], 0, "-1 [-6] .");
    test_cutoff_floating_bit!(-1, -6, Floor, D::ALLONES, vec![], 0, "-1 . [-6]");
    test_cutoff_fixed_position!(-1, -6, Floor, D::ALLONES, vec![], 0, "-1 (-6) .");
    test_cutoff_floating_position!(-1, -6, Floor, D::ALLONES, vec![], 0, "-1 . (-6)");

    let two_nums = u128::from(&BigFixedVec::<D>::construct(D::ZERO, vec![D {value: 127}, D {value: 127}], Position(0)).unwrap());
    test_cutoff_fixed_bit!(two_nums, 0, Floor, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 [0] . f");
    test_cutoff_fixed_bit!(two_nums, 0, Ceiling, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 [0] . c");
    test_cutoff_fixed_bit!(two_nums, 0, Round, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 [0] . r");
    test_cutoff_floating_bit!(two_nums, 0, Floor, D::ZERO, vec![D {value: 64}], 1, "11 . [0] f"); // 0b01111111 to 0b01000000
    test_cutoff_floating_bit!(two_nums, 0, Ceiling, D::ZERO, vec![D {value: 128}], 1, "11 . [0] c");
    test_cutoff_floating_bit!(two_nums, 0, Round, D::ZERO, vec![D {value: 128}], 1, "11 . [0] r");
    test_cutoff_fixed_position!(two_nums, 0, Floor, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 (0) . f");
    test_cutoff_fixed_position!(two_nums, 0, Ceiling, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 (0) . c");
    test_cutoff_fixed_position!(two_nums, 0, Round, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 (0) . r");
    test_cutoff_floating_position!(two_nums, 0, Floor, D::ZERO, vec![D {value: 64}], 1, "11 . (0) f");
    test_cutoff_floating_position!(two_nums, 0, Ceiling, D::ZERO, vec![D {value: 128}], 1, "11 . (0) c"); // 0b01|111111 to 0.10|000000
    test_cutoff_floating_position!(two_nums, 0, Round, D::ZERO, vec![D {value: 128}], 1, "11 . (0) r");
    test_cutoff_fixed_bit!(two_nums, 5, Floor, D::ZERO, vec![D {value: 127} & (D::ALLONES << 5usize), D {value: 127}], 0, "11 [5] . f"); // 0b01|11111|1
    test_cutoff_fixed_bit!(two_nums, 5, Ceiling, D::ZERO, vec![D {value: 128}, D {value: 127}], 0, "11 [5] . c");
    test_cutoff_fixed_bit!(two_nums, 5, Round, D::ZERO, vec![D {value: 128}, D {value: 127}], 0, "11 [5] . r");
    test_cutoff_floating_bit!(two_nums, 5, Floor, D::ZERO, vec![D {value: 127} & !D::ONE], 1, "11 . [5] f"); // 127 is 0b01111111, taking five nontrivial bits gives 0b01111110
    test_cutoff_floating_bit!(two_nums, 5, Ceiling, D::ZERO, vec![D {value: 128}], 1, "11 . [5] c");
    test_cutoff_floating_bit!(two_nums, 5, Round, D::ZERO, vec![D {value: 128}], 1, "11 . [5] r");
    test_cutoff_fixed_position!(two_nums, 5, Floor, D::ZERO, vec![], 0, "11 (5) . f");
    test_cutoff_fixed_position!(two_nums, 5, Ceiling, D::ZERO, vec![D::ONE], 5, "11 (5) . c");
    test_cutoff_fixed_position!(two_nums, 5, Round, D::ZERO, vec![], 0, "11 (5) . r");
    test_cutoff_floating_position!(two_nums, 5, Floor, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 . (5) f");
    test_cutoff_floating_position!(two_nums, 5, Ceiling, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 . (5) c");
    test_cutoff_floating_position!(two_nums, 5, Round, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 . (5) r");
    test_cutoff_fixed_bit!(two_nums, -5, Floor, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 [-5] . f");
    test_cutoff_fixed_bit!(two_nums, -5, Ceiling, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 [-5] . c");
    test_cutoff_fixed_bit!(two_nums, -5, Round, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 [-5] . r");
    test_cutoff_floating_bit!(two_nums, -5, Floor, D::ZERO, vec![D {value: 64}], 1, "11 . [-5] f");
    test_cutoff_floating_bit!(two_nums, -5, Ceiling, D::ZERO, vec![D {value: 128}], 1, "11 . [-5] c");
    test_cutoff_floating_bit!(two_nums, -5, Round, D::ZERO, vec![D {value: 128}], 1, "11 . [-5] r");
    test_cutoff_fixed_position!(two_nums, -5, Floor, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 (-5) . f");
    test_cutoff_fixed_position!(two_nums, -5, Ceiling, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 (-5) . c");
    test_cutoff_fixed_position!(two_nums, -5, Round, D::ZERO, vec![D {value: 127}, D {value: 127}], 0, "11 (-5) . r");
    test_cutoff_floating_position!(two_nums, -5, Floor, D::ZERO, vec![D {value: 64}], 1, "11 . (-5) f");
    test_cutoff_floating_position!(two_nums, -5, Ceiling, D::ZERO, vec![D {value: 128}], 1, "11 . (-5) c");
    test_cutoff_floating_position!(two_nums, -5, Round, D::ZERO, vec![D {value: 128}], 1, "11 . (-5) r");
}*/
