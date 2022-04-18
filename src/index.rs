// Index is a unified usize/isize combination. An Index is an isize value and operations check for overflow.

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
    }
};

use crate::{macros::*};

#[derive(Clone, Copy, Eq)]
pub struct Index {
    value: isize
}

impl Index {
    pub fn cast(x: usize) -> isize {
        TryFrom::try_from(x).unwrap()
    }
    pub fn uncast(x: isize) -> usize {
        TryFrom::try_from(x).unwrap()
    }
    pub const ZERO: Index = Index {
        value: 0
    };

    pub fn val(&self) -> isize {
        self.value
    }
    pub fn saturating_unsigned(&self) -> usize {
        max(0, self.value) as usize
    }
    pub fn saturating_nonnegative(&self) -> Index {
        Index::from(self.saturating_unsigned())
    }
    pub fn to(&self, other: &Index) -> Range<isize> {
        self.value..other.value
    }
}

macro_rules! formatter {
    ($fmt_type: ident, $key: expr) => {
        impl fmt::$fmt_type for Index {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $key, self.value)
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

impl Neg for &Index {
    type Output = Index;
    fn neg(self) -> Index {
        Index {
            value: self.value.checked_neg().unwrap()
        }
    }
}

unary!(Neg, neg, Index);

impl Add for &Index {
    type Output = Index;
    fn add(self, other: &Index) -> Index {
        Index {
            value: self.value.checked_add(other.value).unwrap()
        }
    }
}

op_to_op_assign!(
    Add, add,
    AddAssign, add_assign,
    Index, Index
);

impl Add<&usize> for &Index {
    type Output = Index;
    fn add(self, other: &usize) -> Index {
        Index {
            value: self.value.checked_add(Index::cast(*other)).unwrap()
        }
    }
}

op_to_op_assign!(
    Add, add,
    AddAssign, add_assign,
    Index, usize
);

impl Add<&Index> for usize {
    type Output = Index;
    fn add(self, other: &Index) -> Index {
        Index {
            value: Index::cast(self).checked_add(other.value).unwrap()
        }
    }
}

impl Add<Index> for usize {
    type Output = Index;
    fn add(self, other: Index) -> Index {
        Index {
            value: Index::cast(self).checked_add(other.value).unwrap()
        }
    }
}

impl Add<&isize> for &Index {
    type Output = Index;
    fn add(self, other: &isize) -> Index {
        Index {
            value: self.value.checked_add(*other).unwrap()
        }
    }
}

op_to_op_assign!(
    Add, add,
    AddAssign, add_assign,
    Index, isize
);

impl Add<&Index> for isize {
    type Output = Index;
    fn add(self, other: &Index) -> Index {
        Index {
            value: self.checked_add(other.value).unwrap()
        }
    }
}

impl Add<Index> for isize {
    type Output = Index;
    fn add(self, other: Index) -> Index {
        Index {
            value: self.checked_add(other.value).unwrap()
        }
    }
}

impl Sub for &Index {
    type Output = Index;
    fn sub(self, other: &Index) -> Index {
        Index {
            value: self.value.checked_sub(other.value).unwrap()
        }
    }
}

op_to_op_assign!(
    Sub, sub,
    SubAssign, sub_assign,
    Index, Index
);

impl Sub<&Index> for usize {
    type Output = Index;
    fn sub(self, other: &Index) -> Index {
        Index {
            value: Index::cast(self).checked_sub(other.value).unwrap()
        }
    }
}

impl Sub<Index> for usize {
    type Output = Index;
    fn sub(self, other: Index) -> Index {
        Index {
            value: Index::cast(self).checked_sub(other.value).unwrap()
        }
    }
}

impl Sub<&usize> for &Index {
    type Output = Index;
    fn sub(self, other: &usize) -> Index {
        Index {
            value: self.value.checked_sub(Index::cast(*other)).unwrap()
        }
    }
}

op_to_op_assign!(
    Sub, sub,
    SubAssign, sub_assign,
    Index, usize
);

impl Sub<&isize> for &Index {
    type Output = Index;
    fn sub(self, other: &isize) -> Index {
        Index {
            value: self.value.checked_sub(*other).unwrap()
        }
    }
}

op_to_op_assign!(
    Sub, sub,
    SubAssign, sub_assign,
    Index, isize
);

impl Sub<&Index> for isize {
    type Output = Index;
    fn sub(self, other: &Index) -> Index {
        Index {
            value: self.checked_sub(other.value).unwrap()
        }
    }
}

impl Sub<Index> for isize {
    type Output = Index;
    fn sub(self, other: Index) -> Index {
        Index {
            value: self.checked_sub(other.value).unwrap()
        }
    }
}//

impl Mul for &Index {
    type Output = Index;
    fn mul(self, other: &Index) -> Index {
        Index {
            value: self.value.checked_mul(other.value).unwrap()
        }
    }
}

op_to_op_assign!(
    Mul, mul,
    MulAssign, mul_assign,
    Index, Index
);

impl Mul<&Index> for usize {
    type Output = Index;
    fn mul(self, other: &Index) -> Index {
        Index {
            value: Index::cast(self).checked_mul(other.value).unwrap()
        }
    }
}

impl Mul<Index> for usize {
    type Output = Index;
    fn mul(self, other: Index) -> Index {
        Index {
            value: Index::cast(self).checked_mul(other.value).unwrap()
        }
    }
}

impl Mul<&usize> for &Index {
    type Output = Index;
    fn mul(self, other: &usize) -> Index {
        Index {
            value: self.value.checked_mul(Index::cast(*other)).unwrap()
        }
    }
}

op_to_op_assign!(
    Mul, mul,
    MulAssign, mul_assign,
    Index, usize
);

impl Mul<&isize> for &Index {
    type Output = Index;
    fn mul(self, other: &isize) -> Index {
        Index {
            value: self.value.checked_mul(*other).unwrap()
        }
    }
}

op_to_op_assign!(
    Mul, mul,
    MulAssign, mul_assign,
    Index, isize
);

impl Mul<&Index> for isize {
    type Output = Index;
    fn mul(self, other: &Index) -> Index {
        Index {
            value: self.checked_mul(other.value).unwrap()
        }
    }
}

impl Mul<Index> for isize {
    type Output = Index;
    fn mul(self, other: Index) -> Index {
        Index {
            value: self.checked_mul(other.value).unwrap()
        }
    }
}

impl From<isize> for Index {
    fn from(x: isize) -> Index {
        Index {
            value: x
        }
    }
}

impl From<Index> for usize {
    fn from(x: Index) -> usize {
        Index::uncast(x.value)
    }
}

impl From<&Index> for usize {
    fn from(x: &Index) -> usize {
        Index::uncast(x.value)
    }
}

impl From<Index> for isize {
    fn from(x: Index) -> isize {
        x.value
    }
}

impl From<&Index> for isize {
    fn from(x: &Index) -> isize {
        x.value
    }
}

impl From<usize> for Index {
    fn from(x: usize) -> Index {
        Index {
            value: Index::cast(x)
        }
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Index) -> bool {
        self.value == other.value
    }
}

impl PartialEq<usize> for Index {
    fn eq(&self, other: &usize) -> bool {
        self.value == Index::cast(*other)
    }
}

impl PartialEq<Index> for usize {
    fn eq(&self, other: &Index) -> bool {
        Index::cast(*self) == other.value
    }
}

impl PartialEq<isize> for Index {
    fn eq(&self, other: &isize) -> bool {
        self.value == *other
    }
}

impl PartialEq<Index> for isize {
    fn eq(&self, other: &Index) -> bool {
        *self == other.value
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Index) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<usize> for Index {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.value.partial_cmp(&Index::cast(*other))
    }
}

impl PartialOrd<Index> for usize {
    fn partial_cmp(&self, other: &Index) -> Option<Ordering> {
        Index::cast(*self).partial_cmp(&other.value)
    }
}

impl PartialOrd<isize> for Index {
    fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<Index> for isize {
    fn partial_cmp(&self, other: &Index) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Index) -> Ordering {
        self.value.cmp(&other.value)
    }
}
