#[macro_export]
macro_rules! unary {
    ($op: ident, $op_fn_name: ident, $self_type: ty, $fn_name: ident, $output_type: ty, $error_type: ty) => {
        // -&a
        impl $op for &$self_type {
            type Output = Result<$output_type, $error_type>;
            fn $op_fn_name(self) -> Result<$output_type, $error_type> {
                let mut returner = self.clone();
                returner.$fn_name()?;
                Ok(returner)
            }
        }
        // -a
        impl $op for $self_type {
            type Output = Result<$self_type, $error_type>;
            fn $op_fn_name(self) -> Result<$self_type, $error_type> {
                let mut returner = self.clone();
                returner.$fn_name()?;
                Ok(returner)
            }
        }
    };
}
pub(crate) use unary;

#[macro_export]
macro_rules! unary_copy {
    ($op: ident, $op_fn_name: ident, $self_type: ty, $fn_name: ident, $output_type: ty, $error_type: ty) => {
        // -&a
        impl $op for &$self_type {
            type Output = Result<$output_type, $error_type>;
            fn $op_fn_name(self) -> Result<$output_type, $error_type> {
                <$self_type>::$fn_name(*self)
            }
        }
        // -a
        impl $op for $self_type {
            type Output = Result<$self_type, $error_type>;
            fn $op_fn_name(self) -> Result<$self_type, $error_type> {
                <$self_type>::$fn_name(self)
            }
        }
    };
}
pub(crate) use unary_copy;

#[macro_export]
macro_rules! op_assign_to_op {
    // a +.= &b to:
    // a += &b
    // a += b
    // a +. &b
    // &a + &b
    // &a + b
    // a + &b
    // a + b
    (
        $op: ident, $op_fn_name: ident,
        $op_assign: ident, $op_assign_fn_name: ident,
        $self_type: ty, $other_type: ty,
        $result_type: ty, $error_type: ty
    ) => {
        // a += &b
        impl $op_assign<&$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: &$other_type) {
                <$self_type>::$op_assign_fn_name(self, other).unwrap();
            }
        }

        // a += b
        impl $op_assign<$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: $other_type) {
                <$self_type>::$op_assign_fn_name(self, &other).unwrap();
            }
        }
        
        // a +. &b
        impl $self_type {
            pub fn $op_fn_name(&self, other: &$other_type) -> Result<$result_type, $error_type> {
                let mut res = self.clone();
                res.$op_assign_fn_name(other)?;
                Ok(res)
            }
        }

        // &a + &b
        impl $op<&$other_type> for &$self_type {
            type Output = $result_type;
            fn $op_fn_name(self, other: &$other_type) -> $result_type {
                <$self_type>::$op_fn_name(self, other).unwrap()
            }
        }

        // &a + b
        impl $op<$other_type> for &$self_type {
            type Output = $result_type;
            fn $op_fn_name(self, other: $other_type) -> $result_type {
                <$self_type>::$op_fn_name(self, &other).unwrap()
            }
        }

        // a + &b
        impl $op<&$other_type> for $self_type {
            type Output = $result_type;
            fn $op_fn_name(self, other: &$other_type) -> $result_type {
                <$self_type>::$op_fn_name(&self, other).unwrap()
            }
        }

        // a + b
        impl $op<$other_type> for $self_type {
            type Output = $result_type;
            fn $op_fn_name(self, other: $other_type) -> $result_type {
                <$self_type>::$op_fn_name(&self, &other).unwrap()
            }
        }
    };
}
pub(crate) use op_assign_to_op;

#[macro_export]
macro_rules! cutoff_op {
    // op_assign_to_op
    (
        $op: ident, $op_fn_name: ident, $op_c_fn_name: ident,
        $op_assign: ident, $op_assign_fn_name: ident, $op_assign_c_fn_name: ident,
        $self_type: ty, $other_type: ty, $cutoff_type: ty, $cutoff_fn_name: ident,
        $result_type: ty, $error_type: ty
    ) => {
        // &a += &b
        impl $self_type {
            pub fn $op_assign_c_fn_name(&mut self, other: &$other_type, cutoff: $cutoff_type) -> Result<(), $error_type> {
                self.$op_assign_fn_name(other)?;
                self.$cutoff_fn_name(cutoff)
            }
        }
        // a += &b
        impl $op_assign<(&$other_type, $cutoff_type)> for $self_type {
            fn $op_assign_fn_name(&mut self, (other, cutoff): (&$other_type, $cutoff_type)) {
                <$self_type>::$op_assign_c_fn_name(self, other, cutoff).unwrap();
            }
        }

        // a += b
        impl $op_assign<($other_type, $cutoff_type)> for $self_type {
            fn $op_assign_fn_name(&mut self, (other, cutoff): ($other_type, $cutoff_type)) {
                <$self_type>::$op_assign_c_fn_name(self, &other, cutoff).unwrap();
            }
        }
        
        // a +. &b
        impl $self_type {
            pub fn $op_c_fn_name(&self, other: &$other_type, cutoff: $cutoff_type) -> Result<$result_type, $error_type> {
                let mut res = self.$op_fn_name(other)?;
                res.cutoff(cutoff)?;
                Ok(res)
            }
        }

        // &a + &b
        impl $op<(&$other_type, $cutoff_type)> for &$self_type {
            type Output = $result_type;
            fn $op_fn_name(self, (other, cutoff): (&$other_type, $cutoff_type)) -> $result_type {
                <$self_type>::$op_c_fn_name(self, other, cutoff).unwrap()
            }
        }

        // &a + b
        impl $op<($other_type, $cutoff_type)> for &$self_type {
            type Output = $result_type;
            fn $op_fn_name(self, (other, cutoff): ($other_type, $cutoff_type)) -> $result_type {
                <$self_type>::$op_c_fn_name(self, &other, cutoff).unwrap()
            }
        }

        // a + &b
        impl $op<(&$other_type, $cutoff_type)> for $self_type {
            type Output = $result_type;
            fn $op_fn_name(self, (other, cutoff): (&$other_type, $cutoff_type)) -> $result_type {
                <$self_type>::$op_c_fn_name(&self, other, cutoff).unwrap()
            }
        }

        // a + b
        impl $op<($other_type, $cutoff_type)> for $self_type {
            type Output = $result_type;
            fn $op_fn_name(self, (other, cutoff): ($other_type, $cutoff_type)) -> $result_type {
                <$self_type>::$op_c_fn_name(&self, &other, cutoff).unwrap()
            }
        }
    };
    // unary with extension
    (
        $op: ty, $op_fn_name: ident,
        $self_type: ty, $self_fn_name: ident, $self_c_fn_name: ident,
        $cutoff_type: ty, $cutoff_fn_name: ident, $error_type: ty
    ) => {
        impl $self_type {
            pub fn $self_c_fn_name(&mut self, cutoff: $cutoff_type) -> Result<(), $error_type> {
                self.$self_fn_name()?;
                self.$cutoff_fn_name(cutoff)
            }
        }
    };
}
pub(crate) use cutoff_op;

/*
cutoff_op!();
cutoff_op!(Not, not, BigFixed, negate_c, Cutoff, BigFixed, BigFixedError);
*/

#[macro_export]
macro_rules! op_to_op_assign {
    // &a + &b => (&a + b; a + &b; a + b; a += &b; a += b)
    (
        $op: ident, $op_fn_name: ident,
        $op_assign: ident, $op_assign_fn_name: ident,
        $self_type: ty, $other_type: ty,
        $output_type: ty, $error_type: ty
    ) => {
        // &a + b
        impl $op<$other_type> for &$self_type {
            type Output = Result<$output_type, $error_type>;
            fn $op_fn_name(self, other: $other_type) -> Result<$output_type, $error_type> {
                self.$op_fn_name(&other)
            }
        }
        // a + &b
        impl $op<&$other_type> for $self_type {
            type Output = Result<$output_type, $error_type>;
            fn $op_fn_name(self, other: &$other_type) -> Result<$output_type, $error_type> {
                (&self).$op_fn_name(other)
            }
        }
        // a + b
        impl $op<$other_type> for $self_type {
            type Output = Result<$output_type, $error_type>;
            fn $op_fn_name(self, other: $other_type) -> Result<$output_type, $error_type> {
                (&self).$op_fn_name(&other)
            }
        }
        // a += &b
        impl $op_assign<&$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: &$other_type) {
                *self = self.$op_fn_name(other).unwrap()
            }
        }
        // a += b
        impl $op_assign<$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: $other_type) {
                *self = self.$op_fn_name(&other).unwrap()
            }
        }
    };
}
pub(crate) use op_to_op_assign;
