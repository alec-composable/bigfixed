#[macro_export]
macro_rules! unary {
    // -&a => -a
    ($op: ident, $op_fn_name: ident, $self_type: ty) => {
        impl $op for $self_type {
            type Output = $self_type;
            fn $op_fn_name(self) -> $self_type {
                (&self).$op_fn_name()
            }
        }
    };
}

#[macro_export]
macro_rules! op_assign_to_op {
    // a += &b => (a += b; &a + &b; &a + b; a+ &b; a + b)
    ($op: ident, $op_fn_name: ident, $op_assign: ident, $op_assign_fn_name: ident, $self_type: ty, $other_type: ty) => {
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

#[macro_export]
macro_rules! op_to_op_assign {
    // &a + &b => (&a + b; a + &b; a + b; a += &b; a += b)
    ($op: ident, $op_fn_name: ident, $op_assign: ident, $op_assign_fn_name: ident, $self_type: ty, $other_type: ty, $output_type: ty) => {
        // &a + b
        impl $op<$other_type> for &$self_type {
            type Output = $output_type;
            fn $op_fn_name(self, other: $other_type) -> $output_type {
                self.$op_fn_name(&other)
            }
        }
        // a + &b
        impl $op<&$other_type> for $self_type {
            type Output = $output_type;
            fn $op_fn_name(self, other: &$other_type) -> $output_type {
                (&self).$op_fn_name(other)
            }
        }
        // a + b
        impl $op<$other_type> for $self_type {
            type Output = $output_type;
            fn $op_fn_name(self, other: $other_type) -> $output_type {
                (&self).$op_fn_name(&other)
            }
        }
        // a += &b
        impl $op_assign<&$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: &$other_type) {
                *self = self.$op_fn_name(other)
            }
        }
        // a += b
        impl $op_assign<$other_type> for $self_type {
            fn $op_assign_fn_name(&mut self, other: $other_type) {
                *self = self.$op_fn_name(&other)
            }
        }
    };
    ($op: ident, $op_fn_name: ident, $op_assign: ident, $op_assign_fn_name: ident, $self_type: ty, $other_type: ty) => {
        op_to_op_assign!($op, $op_fn_name, $op_assign, $op_assign_fn_name, $self_type, $other_type, $self_type);
    }
    
}
