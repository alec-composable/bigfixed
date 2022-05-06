#[macro_export]
macro_rules! unary {
    // -&a => -a
    ($op: ident, $op_fn_name: ident, $self_type: ty, $error_type: ty) => {
        impl $op for $self_type {
            type Output = Result<$self_type, $error_type>;
            fn $op_fn_name(self) -> Result<$self_type, $error_type> {
                (&self).$op_fn_name()
            }
        }
    };
}
pub(crate) use unary;

#[macro_export]
macro_rules! op_assign_to_op {
    // a += &b => (a += b; &a + &b; &a + b; a+ &b; a + b)
    (
        $op: ident, $op_fn_name: ident,
        $op_assign: ident, $op_assign_fn_name: ident,
        $self_type: ty, $other_type: ty
    ) => {
        // a += b
        impl $op_assign<$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: $other_type) {
                self.$op_assign_fn_name(&other);
            }
        }
        // &a + &b
        impl $op<&$other_type> for &$self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: &$other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
        // &a + b
        impl $op<$other_type> for &$self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: $other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
        // a + &b
        impl $op<&$other_type> for $self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: &$other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
        // a + b
        impl $op<$other_type> for $self_type {
            type Output = $self_type;
            fn $op_fn_name(self, other: $other_type) -> $self_type {
                use $op_assign;
                let mut res = self.clone();
                res.$op_assign_fn_name(other);
                res
            }
        }
    };
}
//pub(crate) use op_assign_to_op;

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
        }// */
    };
}
pub(crate) use op_to_op_assign;

#[macro_export]
macro_rules! cutoff_op {
    (
        $op: ident, $op_fn_name: ident,
        $op_assign: ident, $op_assign_fn_name: ident,
        $self_type: ty, $other_type: ty, $output_type: ty
    ) => {
        impl $op<(&$other_type, Cutoff)> for &$self_type {
            type Output = $output_type;
            fn $op_fn_name(self, (other, cutoff): (&$other_type, Cutoff)) -> $output_type {
                let mut res = self.$op_fn_name(other);
                res.cutoff(cutoff);
                res
            }
        }

        impl $op_assign<(&$other_type, Cutoff)> for $self_type {
            fn $op_assign_fn_name(&mut self, (other, cutoff): (&$other_type, Cutoff)) {
                self.$op_assign_fn_name(other);
                self.cutoff(cutoff);
            }
        }
    };
}

//pub(crate) use cutoff_op;
