/*use bigfixed::{digit::*, Index, BigFixed};

#[test]
fn index_position() {
    let x = BigFixed::construct(ALLONES, vec![5], Index::Position(-5)).unwrap();
    assert_eq!(x[Index::Position(-700)], 0, "tail far");
    assert_eq!(x[Index::Position(-6)], 0, "tail");
    assert_eq!(x[Index::Position(-5)], 5, "body");
    assert_eq!(x[Index::Position(-4)], ALLONES, "head");
    assert_eq!(x[Index::Position(6)], ALLONES, "head far");
}

#[test]
fn index_bit() {
    let x = BigFixed::construct(ALLONES, vec![5], Index::Position(0)).unwrap();
    assert_eq!(x[Index::Bit(-700)], 0, "tail far");
    assert_eq!(x[Index::Bit(-1)], 0, "tail");
    assert_eq!(x[Index::Bit(0)], 1, "body 0");
    assert_eq!(x[Index::Bit(1)], 0, "body 1");
    assert_eq!(x[Index::Bit(2)], 1, "body 2");
    assert_eq!(x[Index::Bit(3)], 0, "body 3");
    assert_eq!(x[Index::Bit(DIGITBITS as isize)], 1, "head");
    assert_eq!(x[Index::Bit(DIGITBITS as isize * 700)], 1, "head far");
}

#[test]
fn index_mut_position() {
    let mut x = BigFixed::ZERO.clone();
    x[Index::Position(0)] = 1;
    assert_eq!(x, BigFixed::from(1), "zero to one");
    x[Index::Position(0)] = 0;
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![0],
        position: Index::Position(0)
    }).unwrap(), "one to zero");
    x[Index::Position(3)] = 1;
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![0, 0, 0, 1],
        position: Index::Position(0)
    }).unwrap(), "into head");
    x[Index::Position(-2)] = ALLONES;
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![ALLONES, 0, 0, 0, 0, 1],
        position: Index::Position(-2)
    }).unwrap(), "into tail");
}

// gives a reference to the digit in the corresponding position, not to the bit itself
#[test]
fn index_mut_bit() {
    let mut x = BigFixed::from(0);
    x[Index::Bit(0)] = 1;
    assert_eq!(x, BigFixed::from(1), "zero to one");
    x[Index::Bit(0)] = 0;
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![0],
        position: Index::Position(0)
    }).unwrap(), "one to zero");
    x[Index::Bit(3 * DIGITBITS as isize + 3)] = 1;
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![0, 0, 0, 1],
        position: Index::Position(0)
    }).unwrap(), "into head");
    x[Index::Bit(-2 * DIGITBITS as isize)] = ALLONES;
    assert!(x.full_eq(&BigFixed {
        head: 0,
        body: vec![ALLONES, 0, 0, 0, 0, 1],
        position: Index::Position(-2)
    }).unwrap(), "into tail");
}

#[test]
fn set_bit() {
    let mut x = BigFixed::from(-1);
    x.set_bit(4, 0);
    assert!(x.full_eq(&BigFixed {
        head: ALLONES,
        body: vec![!16],
        position: Index::Position(0)
    }).unwrap(), "one bit");
    x.set_bit(4, 1);
    assert!(x.full_eq(&BigFixed {
        head: ALLONES,
        body: vec![ALLONES],
        position: Index::Position(0)
    }).unwrap(), "one bit");
    x.set_bit(-1, 0);
    assert!(x.full_eq(&BigFixed {
        head: ALLONES,
        body: vec![0, ALLONES],
        position: Index::Position(-1)
    }).unwrap(), "one bit");
}*/
