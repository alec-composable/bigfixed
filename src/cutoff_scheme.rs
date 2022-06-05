pub use crate::{digit::*, Index, Cutoff, BigFixed, BigFixedError, scheme_op};

use paste::paste;

use std::{
    ops::{
        Add, AddAssign,
        BitAnd, BitAndAssign,
        BitOr, BitOrAssign,
        BitXor, BitXorAssign,
        Mul, MulAssign,
        Neg, Not,
        Shl, ShlAssign,
        Shr, ShrAssign,
        Sub, SubAssign
    },
    cmp::{Ordering},
    fmt
};

#[derive(Clone, Copy, Debug)]
pub struct CutoffScheme {
    pub comparisons: Cutoff,
    pub arithmetic: Cutoff,
}

#[derive(Clone, Debug)]
pub struct CutoffBoundBigFixed<'a> {
    pub scheme: &'a CutoffScheme,
    pub value: BigFixed
}

impl CutoffScheme {
    pub fn claim(&self, x: BigFixed) -> CutoffBoundBigFixed {
        CutoffBoundBigFixed {
            scheme: self,
            value: x
        }
    }
}

impl<'a> CutoffScheme {
    pub fn construct(&'a self, head: Digit, body: Vec<Digit>, position: Index) -> Result<CutoffBoundBigFixed<'a>, BigFixedError> {
        Ok(
            CutoffBoundBigFixed::<'a> {
                scheme: &self,
                value: BigFixed::construct(head, body, position)?
            }
        )
    }
}

#[macro_export]
macro_rules! big_fixed_from {
    ($scheme: expr, $($i: expr)*) => {
        CutoffBoundBigFixed {
            scheme: &$scheme,
            value: BigFixed::from($($i)*)
        }
    };
}

impl<'a> CutoffBoundBigFixed<'a> {
    pub fn negate(&mut self) -> Result<(), BigFixedError> {
        self.value.negate_c(self.scheme.arithmetic)
    }

    pub fn abs(&self) -> Result<CutoffBoundBigFixed, BigFixedError> {
        Ok(
            CutoffBoundBigFixed {
                scheme: self.scheme,
                value: self.value.abs_c(self.scheme.arithmetic)?
            }
        )
    }
}

macro_rules! call_scheme_op {
    ($op: ident, $op_fn_name: ident) => {
        paste!{
            scheme_op!(
                'a, CutoffBoundBigFixed<'a>, CutoffBoundBigFixed<'a>, 
                $op_fn_name, [<$op_fn_name _assign>], [<$op_fn_name _c>], [<$op_fn_name _assign_c>], [<$op_fn_name _s>], [<$op_fn_name _assign_s>],
                BigFixed, BigFixedError, value, scheme, arithmetic, $op, [<$op Assign>]
            );
        }
    };
    ($op: ident, $op_fn_name: ident, $other_type: ty) => {
        paste!{
            scheme_op!(
                'a, CutoffBoundBigFixed<'a>, CutoffBoundBigFixed<'a>, 
                $op_fn_name, [<$op_fn_name _assign>], [<$op_fn_name _c>], [<$op_fn_name _assign_c>],
                $other_type, BigFixedError, value, scheme, arithmetic, $op, [<$op Assign>]
            );
        }
    };
}

call_scheme_op!(Add, add);
call_scheme_op!(BitAnd, bitand);
call_scheme_op!(BitOr, bitor);
call_scheme_op!(BitXor, bitxor);
call_scheme_op!(Mul, mul);
scheme_op!('a, CutoffBoundBigFixed<'a>, CutoffBoundBigFixed<'a>, Neg, neg, negate);
scheme_op!('a, CutoffBoundBigFixed<'a>, CutoffBoundBigFixed<'a>, Not, not, negate);
call_scheme_op!(Shl, shl, usize);
call_scheme_op!(Shr, shr, usize);
call_scheme_op!(Sub, sub);

impl<'a> PartialEq<BigFixed> for CutoffBoundBigFixed<'a> {
    fn eq(&self, other: &BigFixed) -> bool {
        self.value.full_eq_c(other, self.scheme.comparisons).unwrap()
    }
}

impl<'a> PartialEq for CutoffBoundBigFixed<'a> {
    fn eq(&self, other: &CutoffBoundBigFixed<'a>) -> bool {
        self == &other.value
    }
}

impl<'a> Eq for CutoffBoundBigFixed<'a> {}

impl<'a> PartialOrd<BigFixed> for CutoffBoundBigFixed<'a> {
    fn partial_cmp(&self, other: &BigFixed) -> Option<Ordering> {
        self.value.partial_cmp_c(other, self.scheme.comparisons)
    }
}

impl<'a> PartialOrd for CutoffBoundBigFixed<'a> {
    fn partial_cmp(&self, other: &CutoffBoundBigFixed<'a>) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl<'a> fmt::Display for CutoffBoundBigFixed<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a> fmt::Binary for CutoffBoundBigFixed<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.value)
    }
}

