// BigFixeds can be indexed by position (wrt Digit) or bit. If bit precision is not possible it may convert to the corresponding position index.

pub use std::{
    convert::{
        From,
        TryFrom
    },
    fmt,
    ops::{
        Add, AddAssign,
        Neg,
        Sub, SubAssign,
        Mul, MulAssign,
        Range
    },
    cmp::{
        PartialEq,
        PartialOrd,
        Ordering,
        max
    },
    num::{
        TryFromIntError
    },
    error::{
        Error
    }
};

use crate::{digit::*, macros::*};

#[derive(Clone, Copy, Eq)]
pub enum Index {
    Position(isize),
    Bit(isize)
}

pub use Index::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum IndexError {
    AdditionOverflow,
    MultiplicationOverflow,
    IntegerCastOverflow
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for IndexError {}

impl From<TryFromIntError> for IndexError {
    fn from(_x: TryFromIntError) -> IndexError {
        IntegerCastOverflow
    } 
}

pub use IndexError::*;

impl Index {
    // fails for very large inputs
    pub fn castsize(x: usize) -> Result<isize, IndexError> {
        Ok(TryFrom::try_from(x)?)
    }
    // fails for negative inputs
    pub fn uncastsize(x: isize) -> Result<usize, IndexError> {
        Ok(TryFrom::try_from(x)?)
    }
    // fails for very large inputs
    pub fn position_to_bit(x: isize) -> Result<isize, IndexError> {
        x.checked_mul(DIGITBITS as isize).ok_or(MultiplicationOverflow)
    }

    pub fn bit_to_position(x: isize) -> isize {
        x.div_euclid(DIGITBITS as isize)
    }

    pub fn saturating_unsigned(x: isize) -> usize {
        Index::uncastsize(max(0, x)).unwrap()
    }

    pub fn cast(&self) -> Result<Index, IndexError> {
        match self {
            Position(x) => Ok(Bit(Index::position_to_bit(*x)?)),
            Bit(x) => Ok(Bit(Index::bit_to_position(*x)))
        }
    }

    pub fn cast_to_position(&self) -> Index {
        match self {
            Position(_) => *self,
            Bit(x) => Position(Index::bit_to_position(*x))
        }
    }

    pub fn bit_position_excess(&self) -> isize {
        match self {
            Position(_) => 0,
            Bit(x) => x.rem_euclid(DIGITBITS as isize)
        }
    }

    pub fn cast_to_bit(&self) -> Result<Index, IndexError> {
        match self {
            Position(x) => Ok(Bit(Index::position_to_bit(*x)?)),
            Bit(_) => Ok(*self)
        }
    }

    pub fn saturating_nonnegative(&self) -> Index {
        match self {
            Position(x) => Position(max(0, *x)),
            Bit(x) => Bit(max(0, *x))
        }
    }

    pub fn value(&self) -> isize {
        isize::from(self)
    }

    pub fn unsigned_value(&self) -> usize {
        Index::saturating_unsigned(self.value())
    }

    pub fn bit_value(&self) -> Result<isize, IndexError> {
        return Ok(self.cast_to_bit()?.value())
    }
}

// format convention: (position), [bit], so 2[(0)] == 2, 2[(1)] == 0, 2[[0]] = 0, 2[[1]] == 1

macro_rules! formatter {
    ($fmt_type: ident, $key: expr) => {
        impl fmt::$fmt_type for Index {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Position(x) => {
                        write!(f, "(").ok();
                        write!(f, $key, x).ok();
                        write!(f, ")")
                    }, Bit(x) => {
                        write!(f, "[").ok();
                        write!(f, $key, x).ok();
                        write!(f, "]")
                    }
                }
            }
        }
    };
}

formatter!(Display, "{}");
formatter!(Debug, "{:?}");
formatter!(Octal, "{:o}");
formatter!(LowerHex, "{:x}");
formatter!(UpperHex, "{:X}");
formatter!(Binary, "{:b}");

impl From<&Index> for isize {
    fn from(a: &Index) -> isize {
        match a {
            Position(x) => *x,
            Bit(x) => *x
        }
    }
}

impl From<Index> for isize {
    fn from(a: Index) -> isize {
        isize::from(&a)
    }
}

impl From<&Index> for usize {
    fn from(a: &Index) -> usize {
        match a.saturating_nonnegative() {
            Position(x) => x as usize,
            Bit(x) => x as usize
        }
    }
}

impl From<Index> for usize {
    fn from(a: Index) -> usize {
        usize::from(&a)
    }
}

// fails only for the most negative number
impl Neg for &Index {
    type Output = Result<Index, IndexError>;
    fn neg(self) -> Result<Index, IndexError> {
        match self {
            Position(x) => Ok(Position(x.checked_neg().ok_or(IntegerCastOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_neg().ok_or(IntegerCastOverflow)?))
        }
    }
}

unary!(Neg, neg, Index, IndexError);

impl Add for &Index {
    type Output = Result<Index, IndexError>;
    fn add(self, other: &Index) -> Result<Index, IndexError> {
        let a;
        let b;
        let position;
        match (self, other) {
            (Position(x), Position(y)) => {
                a = *x;
                b = *y;
                position = true;
            },
            (Bit(x), Bit(y)) => {
                a = *x;
                b = *y;
                position = false;
            },
            (Position(x), Bit(y)) => {
                a = Index::position_to_bit(*x)?;
                b = *y;
                position = false;
            },
            (Bit(x), Position(y)) => {
                a = *x;
                b = Index::position_to_bit(*y)?;
                position = false;
            }
        };
        let sum = a.checked_add(b).ok_or(AdditionOverflow)?;
        if position {
            Ok(Position(sum))
        } else {
            Ok(Bit(sum))
        }
    }
}

op_to_op_assign!(
    Add, add,
    AddAssign, add_assign,
    Index, Index,
    Index, IndexError
);

impl Add<&usize> for &Index {
    type Output = Result<Index, IndexError>;
    fn add(self, other: &usize) -> Result<Index, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_add(Index::castsize(*other)?).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_add(Index::castsize(*other)?).ok_or(AdditionOverflow)?))
            }
        }
    }
}

op_to_op_assign!(
    Add, add,
    AddAssign, add_assign,
    Index, usize,
    Index, IndexError
);

impl Add<&Index> for usize {
    type Output = Result<Index, IndexError>;
    fn add(self, other: &Index) -> Result<Index, IndexError> {
        match other {
            Position(x) => Ok(Position(x.checked_add(Index::castsize(self)?).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_add(Index::castsize(self)?).ok_or(AdditionOverflow)?)),
        }
    }
}

impl Add<Index> for usize {
    type Output = Result<Index, IndexError>;
    fn add(self, other: Index) -> Result<Index, IndexError> {
        self + &other
    }
}

impl Add<&isize> for &Index {
    type Output = Result<Index, IndexError>;
    fn add(self, other: &isize) -> Result<Index, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_add(*other).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_add(*other).ok_or(AdditionOverflow)?))
            }
        }
    }
}

op_to_op_assign!(
    Add, add,
    AddAssign, add_assign,
    Index, isize,
    Index, IndexError
);

impl Add<&Index> for isize {
    type Output = Result<Index, IndexError>;
    fn add(self, other: &Index) -> Result<Index, IndexError> {
        match other {
            Position(x) => Ok(Position(self.checked_add(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(self.checked_add(*x).ok_or(AdditionOverflow)?))
        }
    }
}

impl Add<Index> for isize {
    type Output = Result<Index, IndexError>;
    fn add(self, other: Index) -> Result<Index, IndexError> {
        self + &other
    }
}

impl Sub for &Index {
    type Output = Result<Index, IndexError>;
    fn sub(self, other: &Index) -> Result<Index, IndexError> {
        let a;
        let b;
        let position;
        match (self, other) {
            (Position(x), Position(y)) => {
                a = *x;
                b = *y;
                position = true;
            },
            (Bit(x), Bit(y)) => {
                a = *x;
                b = *y;
                position = false;
            },
            (Position(x), Bit(y)) => {
                a = Index::position_to_bit(*x)?;
                b = *y;
                position = false;
            },
            (Bit(x), Position(y)) => {
                a = *x;
                b = Index::position_to_bit(*y)?;
                position = false;
            }
        };
        let diff = a.checked_sub(b).ok_or(AdditionOverflow)?;
        if position {
            Ok(Position(diff))
        } else {
            Ok(Bit(diff))
        }
    }
}

op_to_op_assign!(
    Sub, sub,
    SubAssign, sub_assign,
    Index, Index,
    Index, IndexError
);

impl Sub<&usize> for &Index {
    type Output = Result<Index, IndexError>;
    fn sub(self, other: &usize) -> Result<Index, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_sub(Index::castsize(*other)?).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_sub(Index::castsize(*other)?).ok_or(AdditionOverflow)?))
            }
        }
    }
}

op_to_op_assign!(
    Sub, sub,
    SubAssign, sub_assign,
    Index, usize,
    Index, IndexError
);

impl Sub<&Index> for usize {
    type Output = Result<Index, IndexError>;
    fn sub(self, other: &Index) -> Result<Index, IndexError> {
        match other {
            Position(x) => Ok(Position(Index::castsize(self)?.checked_sub(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(Index::castsize(self)?.checked_sub(*x).ok_or(AdditionOverflow)?)),
        }
    }
}

impl Sub<Index> for usize {
    type Output = Result<Index, IndexError>;
    fn sub(self, other: Index) -> Result<Index, IndexError> {
        self - &other
    }
}

impl Sub<&isize> for &Index {
    type Output = Result<Index, IndexError>;
    fn sub(self, other: &isize) -> Result<Index, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_sub(*other).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_sub(*other).ok_or(AdditionOverflow)?))
            }
        }
    }
}

op_to_op_assign!(
    Sub, sub,
    SubAssign, sub_assign,
    Index, isize,
    Index, IndexError
);

impl Sub<&Index> for isize {
    type Output = Result<Index, IndexError>;
    fn sub(self, other: &Index) -> Result<Index, IndexError> {
        match other {
            Position(x) => Ok(Position(self.checked_sub(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(self.checked_sub(*x).ok_or(AdditionOverflow)?))
        }
    }
}

impl Sub<Index> for isize {
    type Output = Result<Index, IndexError>;
    fn sub(self, other: Index) -> Result<Index, IndexError> {
        self - &other
    }
}

impl Mul for &Index {
    type Output = Result<Index, IndexError>;
    fn mul(self, other: &Index) -> Result<Index, IndexError> {
        let a;
        let b;
        let position;
        match (self, other) {
            (Position(x), Position(y)) => {
                a = *x;
                b = *y;
                position = true;
            },
            (Bit(x), Bit(y)) => {
                a = *x;
                b = *y;
                position = false;
            },
            (Position(x), Bit(y)) => {
                a = Index::position_to_bit(*x)?;
                b = *y;
                position = false;
            },
            (Bit(x), Position(y)) => {
                a = *x;
                b = Index::position_to_bit(*y)?;
                position = false;
            }
        };
        let prod = a.checked_mul(b).ok_or(AdditionOverflow)?;
        if position {
            Ok(Position(prod))
        } else {
            Ok(Bit(prod))
        }
    }
}

op_to_op_assign!(
    Mul, mul,
    MulAssign, mul_assign,
    Index, Index,
    Index, IndexError
);

impl Mul<&usize> for &Index {
    type Output = Result<Index, IndexError>;
    fn mul(self, other: &usize) -> Result<Index, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_mul(Index::castsize(*other)?).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_mul(Index::castsize(*other)?).ok_or(AdditionOverflow)?))
            }
        }
    }
}

op_to_op_assign!(
    Mul, mul,
    MulAssign, mul_assign,
    Index, usize,
    Index, IndexError
);

impl Mul<&Index> for usize {
    type Output = Result<Index, IndexError>;
    fn mul(self, other: &Index) -> Result<Index, IndexError> {
        match other {
            Position(x) => Ok(Position(x.checked_mul(Index::castsize(self)?).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_mul(Index::castsize(self)?).ok_or(AdditionOverflow)?)),
        }
    }
}

impl Mul<Index> for usize {
    type Output = Result<Index, IndexError>;
    fn mul(self, other: Index) -> Result<Index, IndexError> {
        self * &other
    }
}

impl Mul<&isize> for &Index {
    type Output = Result<Index, IndexError>;
    fn mul(self, other: &isize) -> Result<Index, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_mul(*other).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_mul(*other).ok_or(AdditionOverflow)?))
            }
        }
    }
}

op_to_op_assign!(
    Mul, mul,
    MulAssign, mul_assign,
    Index, isize,
    Index, IndexError
);

impl Mul<&Index> for isize {
    type Output = Result<Index, IndexError>;
    fn mul(self, other: &Index) -> Result<Index, IndexError> {
        match other {
            Position(x) => Ok(Position(self.checked_mul(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(self.checked_mul(*x).ok_or(AdditionOverflow)?))
        }
    }
}

impl Mul<Index> for isize {
    type Output = Result<Index, IndexError>;
    fn mul(self, other: Index) -> Result<Index, IndexError> {
        self * &other
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Index) -> bool {
        match (self, other) {
            (Position(x), Position(y)) => x == y,
            (Bit(x), Bit(y)) => x == y,
            (Position(x), Bit(y)) => {
                *x == Index::bit_to_position(*y)
            },
            (Bit(x), Position(y)) => {
                *y == Index::bit_to_position(*x)
            }
        }
    }
}

impl PartialEq<isize> for Index {
    fn eq(&self, other: &isize) -> bool {
        match self {
            Position(x) => *x == *other,
            Bit(x) => *x == *other
        }
    }
}

impl PartialEq<usize> for Index {
    fn eq(&self, other: &usize) -> bool {
        let o = Index::castsize(*other).unwrap();
        match self {
            Position(x) => *x == o,
            Bit(x) => *x == o
        }
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Index) -> Option<Ordering> {
        match (self, other) {
            (Position(x), Position(y)) => x.partial_cmp(y),
            (Bit(x), Bit(y)) => x.partial_cmp(y),
            (Position(x), Bit(y)) => {
                match Index::position_to_bit(*x) {
                    Ok(z) => z.partial_cmp(y),
                    Err(_) => None
                }
            },
            (Bit(x), Position(y)) => {
                match Index::position_to_bit(*y) {
                    Ok(z) => x.partial_cmp(&z),
                    Err(_) => None
                }
            }
        }
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Index) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd<isize> for Index {
    fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
        match self {
            Position(x) => x.partial_cmp(other),
            Bit(x) => x.partial_cmp(other)
        }
    }
}

impl PartialOrd<usize> for Index {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        let o = &Index::castsize(*other).unwrap();
        match self {
            Position(x) => x.partial_cmp(o),
            Bit(x) => x.partial_cmp(o)
        }
    }
}

