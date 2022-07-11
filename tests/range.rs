use bigfixed::*;

type D = u64;

#[test]
fn ensure_valid_range() {
    let padded = BigFixed {
        head: D::ALLONES,
        body: vec![0, 0, 6, D::ALLONES, D::ALLONES],
        position: Position(-6)
    };
    let mut no_growth = padded.clone();
    no_growth.ensure_valid_range(Position(0), Position(0)).unwrap();
    assert_eq!(padded, no_growth, "no growth internal");
    no_growth.ensure_valid_range(no_growth.position, (no_growth.position + 1isize).unwrap()).unwrap();
    assert_eq!(padded, no_growth, "no growth start");
    no_growth.ensure_valid_range((no_growth.body_high().unwrap() - 1isize).unwrap(), no_growth.body_high().unwrap()).unwrap();
    assert_eq!(padded, no_growth, "no growth end");
    no_growth.ensure_valid_range(no_growth.position, no_growth.body_high().unwrap()).unwrap();
    assert_eq!(padded, no_growth, "no growth full");
    let mut growth = BigFixed {
        head: D::ALLONES,
        body: vec![6],
        position: Position(-4)
    };
    growth.ensure_valid_range(Bit(-5*(D::DIGITBITS as isize) - 1), Position(-3)).unwrap();
    assert_eq!(growth, BigFixed {
        head: D::ALLONES,
        body: vec![0, 0 , 6],
        position: Position(-6)
    }, "bit growth below");
    growth.ensure_valid_range(Bit(-3*(D::DIGITBITS as isize) - 3), Position(-2)).unwrap();
    assert_eq!(growth, BigFixed {
        head: D::ALLONES,
        body: vec![0, 0 , 6, D::ALLONES],
        position: Position(-6)
    }, "bit growth above");
    growth.ensure_valid_range(Position(-6), Position(-1)).unwrap();
    assert_eq!(growth, padded, "position growth both ends");
    let mut x = BigFixed {
        head: D::ALLONES,
        body: vec![],
        position: Position(1)
    };
    x.ensure_valid_position(Position(-1)).unwrap();
    assert_eq!(x, BigFixed {
        head: D::ALLONES,
        body: vec![0, 0],
        position: Position(-1)
    }, "negative no body to body below");
    x = BigFixed {
        head: D::ALLONES,
        body: vec![],
        position: Position(-1)
    };
    x.ensure_valid_position(Position(1)).unwrap();
    assert_eq!(x, BigFixed {
        head: D::ALLONES,
        body: vec![D::ALLONES, D::ALLONES, D::ALLONES],
        position: Position(-1)
    }, "negative no body to body above");
}

#[test]
fn shift() {
    assert_eq!(
        BigFixed::<D> {
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
        BigFixed::<D> {
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
        BigFixed::<D> {
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
            head: D::ALLONES,
            body: vec![],
            position: Position(0)
        }.shift(Position(100)).unwrap(),
        BigFixed {
            head: D::ALLONES,
            body: vec![],
            position: Position(100)
        },
        "-1 shifting left 100 positions"
    );
    assert_eq!(
        BigFixed {
            head: D::ALLONES,
            body: vec![1, D::ALLONES, 0 , 3],
            position: Bit(4)
        }.shift(Bit(-2)).unwrap(),
        BigFixed {
            head: D::ALLONES,
            body: vec![1 << 2, D::ALLONES << 2, 3, 3 << 2, D::ALLONES << 2],
            position: Position(0)
        },
        "negative multinumber 1"
    );
}