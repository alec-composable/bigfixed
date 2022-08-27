use bigfixed::*;
use Index::{Bit, Position};
use Rounding::*;

type D = u8;

fn fixed(fixed: isize, round: Rounding) -> Cutoff<D> {
    Cutoff {
        fixed: Some(Position(fixed)),
        floating: None,
        round: round
    }
}

fn fixed_b(fixed: isize, round: Rounding) -> Cutoff<D> {
    Cutoff {
        fixed: Some(Bit(fixed)),
        floating: None,
        round: round
    }
}

fn floating(floating: isize, round: Rounding) -> Cutoff<D> {
    Cutoff {
        fixed: None,
        floating: Some(Position(floating)),
        round: round
    }
}

fn floating_b(floating: isize, round: Rounding) -> Cutoff<D> {
    Cutoff {
        fixed: None,
        floating: Some(Bit(floating)),
        round: round
    }
}

#[test]
fn cutoff_index() {
    cutoff_index_result().unwrap();
}

fn cutoff_index_result() -> Result<(), BigFixedVecError> {
    // 00000000 00000011.10000001 10000000
    let a: BigFixedVec<D> = BigFixedVec {
        head: 0,
        body: vec![0b10000000, 0b10000001, 0b00000011],
        position: Position(-2)
    };

    assert_eq!(a.greatest_bit_position()?, Bit(1));

    // 11111111 11110001.10000001 10000000
    let b: BigFixedVec<D> = BigFixedVec {
        head: D::ALLONES,
        body: vec![0b10000000, 0b10000001, 0b11110001],
        position: Position(-2)
    };

    assert_eq!(b.greatest_bit_position()?, Bit(3));

    assert_eq!(a.cutoff_index(fixed(0, Floor))?, Position(0));
    assert_eq!(b.cutoff_index(fixed(0, Floor))?, Position(0));
    assert_eq!(a.cutoff_index(fixed_b(0, Floor))?, Bit(0));
    assert_eq!(b.cutoff_index(fixed_b(0, Floor))?, Bit(0));

    assert_eq!(a.cutoff_index(fixed(10, Floor))?, Position(10));
    assert_eq!(b.cutoff_index(fixed(10, Floor))?, Position(10));
    assert_eq!(a.cutoff_index(fixed_b(10, Floor))?, Bit(10));
    assert_eq!(b.cutoff_index(fixed_b(10, Floor))?, Bit(10));

    assert_eq!(a.cutoff_index(fixed(-4, Floor))?, Position(-4));
    assert_eq!(b.cutoff_index(fixed(-4, Floor))?, Position(-4));
    assert_eq!(a.cutoff_index(fixed_b(-4, Floor))?, Bit(-4));
    assert_eq!(b.cutoff_index(fixed_b(-4, Floor))?, Bit(-4));

    assert_eq!(a.cutoff_index(floating(1, Floor))?, Position(-1));
    assert_eq!(b.cutoff_index(floating(1, Floor))?, Position(-1));
    assert_eq!(a.cutoff_index(floating_b(1, Floor))?, Bit(0));
    assert_eq!(b.cutoff_index(floating_b(1, Floor))?, Bit(2));

    assert_eq!(a.cutoff_index(floating(5, Floor))?, Position(-5));
    assert_eq!(b.cutoff_index(floating(5, Floor))?, Position(-5));
    assert_eq!(a.cutoff_index(floating_b(5, Floor))?, Bit(-4));
    assert_eq!(b.cutoff_index(floating_b(5, Floor))?, Bit(-2));

    Ok(())
}
