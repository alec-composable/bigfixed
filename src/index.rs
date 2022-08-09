/*
    Index comes in two forms: position and bit. Positional depends on what digits are used while Bit does not.
        ... 0000 0010 0110.1100 ... (Digit is the theoretical u4)
    position   2    1    0   -1
    bit        8  6 4  2 0-1-3

    Index also handles conversions between usize and isize.

    Possible sources of errors are integer overflows (very large Positions can't be converted to Bit, extremes for usize/isize conversion, etc)
    and the completely avoidable UsedDigitTypeAsIndex error. For compiler reasons Index has a third type, DigitTypeInUse, which does not actually
    serve as an index but instead defines which digit type is in use. Don't try using DigitTypeInUse as an index.

    Some Index operations are overloaded to work with usize and isize as well. In these cases the number is compared to the value of the Index
    regardless of whether it is Position or Bit. This means that equivalent Indexes (like Position(1) and Bit(8) for u8) can give different results.
*/

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
        Ord,
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

use crate::{digit::Digit, macros::*};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum IndexError {
    AdditionOverflow,
    MultiplicationOverflow,
    IntegerCastOverflow,
    UsedDigitTypeAsIndex
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

#[derive(Clone, Copy, Eq)]
pub enum Index<D: Digit> {
    Position(isize),
    Bit(isize),
    // DigitTypeInUse is to alleviate the "unused parameter" error
    DigitTypeInUse(D)
}

pub use Index::*;

impl<D: Digit> Index<D> {
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
        x.checked_mul(D::DIGITBITS as isize).ok_or(MultiplicationOverflow)
    }

    pub fn bit_to_position(x: isize) -> isize {
        x.div_euclid(D::DIGITBITS as isize)
    }

    pub fn saturating_unsigned(x: isize) -> Result<usize, IndexError> {
        Self::uncastsize(max(0, x))
    }

    pub fn cast_to_position(&self) -> Result<Self, IndexError> {
        match self {
            Position(_) => Ok(*self),
            Bit(x) => Ok(Position(Self::bit_to_position(*x))),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }

    pub fn bit_position_excess(&self) -> Result<usize, IndexError> {
        match self {
            Position(_) => Ok(0),
            Bit(x) => Ok(x.rem_euclid(D::DIGITBITS as isize) as usize),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }

    pub fn cast_to_bit(&self) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => Ok(Bit(Self::position_to_bit(*x)?)),
            Bit(_) => Ok(*self),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }

    pub fn saturating_nonnegative(&self) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => Ok(Position(max(0, *x))),
            Bit(x) => Ok(Bit(max(0, *x))),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }

    pub fn value(&self) -> Result<isize, IndexError> {
        match self {
            Position(x) => Ok(*x),
            Bit(x) => Ok(*x),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }

    pub fn unsigned_value(&self) -> Result<usize, IndexError> {
        Self::saturating_unsigned(self.value()?)
    }

    pub fn bit_value(&self) -> Result<isize, IndexError> {
        return self.cast_to_bit()?.value()
    }

    pub fn position_value(&self) -> Result<isize, IndexError> {
        return self.cast_to_position()?.value()
    }
    
    pub fn neg(self) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => Ok(Position(x.checked_neg().ok_or(IntegerCastOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_neg().ok_or(IntegerCastOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }

    pub fn range(low: Index<D>, high: Index<D>) -> Range<Index<D>> {
        Range {
            start: low,
            end: high
        }
    }
}

/*
    format convention: (position), [bit], so 2[(0)] == 2, 2[(1)] == 0, 2[[0]] = 0, 2[[1]] == 1. Here 2 is ...00000010.00000...
    ... 0000 0010.0000 ...
    (0)      ||||.
    (1) ||||     .
    [0]         |.
    [1]        | .
*/

macro_rules! formatter {
    ($fmt_type: ident, $key: expr) => {
        impl<D: Digit> fmt::$fmt_type for Index<D> {
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
                    }, DigitTypeInUse(d) => {
                        write!(f, "{:?}", d)
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

// these converters do not add anything new but they make it easier to ignore details by just calling .into() in some cases 
impl<D: Digit> From<&Index<D>> for Result<isize, IndexError> {
    fn from(a: &Index<D>) -> Result<isize, IndexError> {
        a.value()
    }
}

impl<D: Digit> From<Index<D>> for Result<isize, IndexError> {
    fn from(a: Index<D>) -> Result<isize, IndexError> {
        a.value()
    }
}

impl<D: Digit> From<&Index<D>> for Result<usize, IndexError> {
    fn from(a: &Index<D>) -> Result<usize, IndexError> {
        Index::<D>::saturating_unsigned(a.value()?)
    }
}

impl<D: Digit> From<Index<D>> for Result<usize, IndexError> {
    fn from(a: Index<D>) -> Result<usize, IndexError> {
        Index::<D>::saturating_unsigned(a.value()?)
    }
}

unary_copy_parametrized!(Neg, neg, D, Digit, Index<D>, neg, Index<D>, IndexError);

impl<D: Digit> Add for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn add(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
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
                a = Index::<D>::position_to_bit(*x)?;
                b = *y;
                position = false;
            },
            (Bit(x), Position(y)) => {
                a = *x;
                b = Index::<D>::position_to_bit(*y)?;
                position = false;
            },
            _ => return Err(UsedDigitTypeAsIndex)
        };
        let sum = a.checked_add(b).ok_or(AdditionOverflow)?;
        if position {
            Ok(Position(sum))
        } else {
            Ok(Bit(sum))
        }
    }
}

op_to_op_assign_parametrized!(
    Add, add,
    AddAssign, add_assign,
    D, Digit,
    Index<D>, Index<D>,
    Index<D>, IndexError
);

impl<D: Digit> Add<&usize> for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn add(self, other: &usize) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_add(Index::<D>::castsize(*other)?).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_add(Index::<D>::castsize(*other)?).ok_or(AdditionOverflow)?))
            },
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

op_to_op_assign_parametrized!(
    Add, add,
    AddAssign, add_assign,
    D, Digit,
    Index<D>, usize,
    Index<D>, IndexError
);

impl<D: Digit> Add<&Index<D>> for usize {
    type Output = Result<Index<D>, IndexError>;
    fn add(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
        match other {
            Position(x) => Ok(Position(x.checked_add(Index::<D>::castsize(self)?).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_add(Index::<D>::castsize(self)?).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

impl<D: Digit> Add<Index<D>> for usize {
    type Output = Result<Index<D>, IndexError>;
    fn add(self, other: Index<D>) -> Result<Index<D>, IndexError> {
        self + &other
    }
}

impl<D: Digit> Add<&isize> for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn add(self, other: &isize) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => {
                Ok(Position(x.checked_add(*other).ok_or(AdditionOverflow)?))
            },
            Bit(x) => {
                Ok(Bit(x.checked_add(*other).ok_or(AdditionOverflow)?))
            },
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

op_to_op_assign_parametrized!(
    Add, add,
    AddAssign, add_assign,
    D, Digit,
    Index<D>, isize,
    Index<D>, IndexError
);

impl<D: Digit> Add<&Index<D>> for isize {
    type Output = Result<Index<D>, IndexError>;
    fn add(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
        match other {
            Position(x) => Ok(Position(self.checked_add(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(self.checked_add(*x).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

impl<D: Digit> Add<Index<D>> for isize {
    type Output = Result<Index<D>, IndexError>;
    fn add(self, other: Index<D>) -> Result<Index<D>, IndexError> {
        self + &other
    }
}

impl<D: Digit> Sub for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn sub(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
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
                a = Index::<D>::position_to_bit(*x)?;
                b = *y;
                position = false;
            },
            (Bit(x), Position(y)) => {
                a = *x;
                b = Index::<D>::position_to_bit(*y)?;
                position = false;
            },
            _ => return Err(UsedDigitTypeAsIndex)
        };
        let diff = a.checked_sub(b).ok_or(AdditionOverflow)?;
        if position {
            Ok(Position(diff))
        } else {
            Ok(Bit(diff))
        }
    }
}

op_to_op_assign_parametrized!(
    Sub, sub,
    SubAssign, sub_assign,
    D, Digit,
    Index<D>, Index<D>,
    Index<D>, IndexError
);

impl<D: Digit> Sub<&usize> for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn sub(self, other: &usize) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => Ok(Position(x.checked_sub(Index::<D>::castsize(*other)?).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_sub(Index::<D>::castsize(*other)?).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

op_to_op_assign_parametrized!(
    Sub, sub,
    SubAssign, sub_assign,
    D, Digit,
    Index<D>, usize,
    Index<D>, IndexError
);

impl<D: Digit> Sub<&Index<D>> for usize {
    type Output = Result<Index<D>, IndexError>;
    fn sub(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
        match other {
            Position(x) => Ok(Position(Index::<D>::castsize(self)?.checked_sub(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(Index::<D>::castsize(self)?.checked_sub(*x).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

impl<D: Digit> Sub<Index<D>> for usize {
    type Output = Result<Index<D>, IndexError>;
    fn sub(self, other: Index<D>) -> Result<Index<D>, IndexError> {
        self - &other
    }
}

impl<D: Digit> Sub<&isize> for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn sub(self, other: &isize) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => Ok(Position(x.checked_sub(*other).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_sub(*other).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

op_to_op_assign_parametrized!(
    Sub, sub,
    SubAssign, sub_assign,
    D, Digit,
    Index<D>, isize,
    Index<D>, IndexError
);

impl<D: Digit> Sub<&Index<D>> for isize {
    type Output = Result<Index<D>, IndexError>;
    fn sub(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
        match other {
            Position(x) => Ok(Position(self.checked_sub(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(self.checked_sub(*x).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

impl<D: Digit> Sub<Index<D>> for isize {
    type Output = Result<Index<D>, IndexError>;
    fn sub(self, other: Index<D>) -> Result<Index<D>, IndexError> {
        self - &other
    }
}

impl<D: Digit> Mul for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn mul(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
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
                a = Index::<D>::position_to_bit(*x)?;
                b = *y;
                position = false;
            },
            (Bit(x), Position(y)) => {
                a = *x;
                b = Index::<D>::position_to_bit(*y)?;
                position = false;
            },
            _ => return Err(UsedDigitTypeAsIndex)
        };
        let prod = a.checked_mul(b).ok_or(AdditionOverflow)?;
        if position {
            Ok(Position(prod))
        } else {
            Ok(Bit(prod))
        }
    }
}

op_to_op_assign_parametrized!(
    Mul, mul,
    MulAssign, mul_assign,
    D, Digit,
    Index<D>, Index<D>,
    Index<D>, IndexError
);

impl<D: Digit> Mul<&usize> for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn mul(self, other: &usize) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => Ok(Position(x.checked_mul(Index::<D>::castsize(*other)?).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_mul(Index::<D>::castsize(*other)?).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

op_to_op_assign_parametrized!(
    Mul, mul,
    MulAssign, mul_assign,
    D, Digit,
    Index<D>, usize,
    Index<D>, IndexError
);

impl<D: Digit> Mul<&Index<D>> for usize {
    type Output = Result<Index<D>, IndexError>;
    fn mul(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
        match other {
            Position(x) => Ok(Position(x.checked_mul(Index::<D>::castsize(self)?).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_mul(Index::<D>::castsize(self)?).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

impl<D: Digit> Mul<Index<D>> for usize {
    type Output = Result<Index<D>, IndexError>;
    fn mul(self, other: Index<D>) -> Result<Index<D>, IndexError> {
        self * &other
    }
}

impl<D: Digit> Mul<&isize> for &Index<D> {
    type Output = Result<Index<D>, IndexError>;
    fn mul(self, other: &isize) -> Result<Index<D>, IndexError> {
        match self {
            Position(x) => Ok(Position(x.checked_mul(*other).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(x.checked_mul(*other).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

op_to_op_assign_parametrized!(
    Mul, mul,
    MulAssign, mul_assign,
    D, Digit,
    Index<D>, isize,
    Index<D>, IndexError
);

impl<D: Digit> Mul<&Index<D>> for isize {
    type Output = Result<Index<D>, IndexError>;
    fn mul(self, other: &Index<D>) -> Result<Index<D>, IndexError> {
        match other {
            Position(x) => Ok(Position(self.checked_mul(*x).ok_or(AdditionOverflow)?)),
            Bit(x) => Ok(Bit(self.checked_mul(*x).ok_or(AdditionOverflow)?)),
            DigitTypeInUse(_) => Err(UsedDigitTypeAsIndex)
        }
    }
}

impl<D: Digit> Mul<Index<D>> for isize {
    type Output = Result<Index<D>, IndexError>;
    fn mul(self, other: Index<D>) -> Result<Index<D>, IndexError> {
        self * &other
    }
}

impl<D: Digit> PartialEq for Index<D> {
    fn eq(&self, other: &Index<D>) -> bool {
        match (self, other) {
            (Position(x), Position(y)) => x == y,
            (Bit(x), Bit(y)) => x == y,
            (Position(x), Bit(y)) => {
                *x == Self::bit_to_position(*y)
            },
            (Bit(x), Position(y)) => {
                *y == Self::bit_to_position(*x)
            },
            (DigitTypeInUse(x), DigitTypeInUse(y)) => x == y,
            _ => false // like NaN != NaN
        }
    }
}

impl<D: Digit> PartialEq<isize> for Index<D> {
    fn eq(&self, other: &isize) -> bool {
        match self {
            Position(x) => *x == *other,
            Bit(x) => *x == *other,
            DigitTypeInUse(_) => false // like NaN != NaN
        }
    }
}

impl<D: Digit> PartialEq<usize> for Index<D> {
    fn eq(&self, other: &usize) -> bool {
        match Self::castsize(*other) {
            Ok(o) => {
                match self {
                    Position(x) => *x == o,
                    Bit(x) => *x == o,
                    DigitTypeInUse(_) => false // like NaN != NaN
                }
            },
            Err(_) => return false
        }
    }
}

impl<D: Digit> PartialOrd for Index<D> {
    fn partial_cmp(&self, other: &Index<D>) -> Option<Ordering> {
        match (self, other) {
            (Position(x), Position(y)) => x.partial_cmp(y),
            (Bit(x), Bit(y)) => x.partial_cmp(y),
            (Position(x), Bit(y)) => {
                match Self::position_to_bit(*x) {
                    Ok(z) => z.partial_cmp(y),
                    Err(_) => None
                }
            },
            (Bit(x), Position(y)) => {
                match Self::position_to_bit(*y) {
                    Ok(z) => x.partial_cmp(&z),
                    Err(_) => None
                }
            }
            _ => None
        }
    }
}

impl<D: Digit> PartialOrd<isize> for Index<D> {
    fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
        match self {
            Position(x) => x.partial_cmp(other),
            Bit(x) => x.partial_cmp(other),
            DigitTypeInUse(_) => None
        }
    }
}

impl<D: Digit> PartialOrd<usize> for Index<D> {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        match Self::castsize(*other) {
            Ok(o) => {
                match self {
                    Position(x) => x.partial_cmp(&o),
                    Bit(x) => x.partial_cmp(&o),
                    DigitTypeInUse(_) => None
                }
            },
            Err(_) => None
        }
    }
}

/*
    While PartialOrd is the correct trait for Index, we implement Ord so we can take max and min of them.
    Unorderable pairs arise only when one of the values is broken (too large or DigitTypeInUse), in which case
    we make the arbitrary choice to say they are equal to everything to get Ord to compile.
    The other choice is to panic on invalid entries.
*/
impl<D: Digit> Ord for Index<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(r) => r,
            None => Ordering::Equal
        }
    }
}
