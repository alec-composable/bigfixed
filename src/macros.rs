#![allow(unused_imports)]

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
macro_rules! unary_copy_parametrized {
    ($op: ident, $op_fn_name: ident, $parameter: ident, $parameter_bound: path, $self_type: ty, $fn_name: ident, $output_type: ty, $error_type: ty) => {
        // -&a
        impl<$parameter: $parameter_bound> $op for &$self_type {
            type Output = Result<$output_type, $error_type>;
            fn $op_fn_name(self) -> Result<$output_type, $error_type> {
                <$self_type>::$fn_name(*self)
            }
        }
        // -a
        impl<$parameter: $parameter_bound> $op for $self_type {
            type Output = Result<$self_type, $error_type>;
            fn $op_fn_name(self) -> Result<$self_type, $error_type> {
                <$self_type>::$fn_name(self)
            }
        }
    };
}
pub(crate) use unary_copy_parametrized;


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

#[macro_export]
macro_rules! scheme_op {
    (
        $lifetime: lifetime, $scheme: ty, $output_type: ty,
        $op: ident, $op_fn_name: ident, $fn_name: ident
    ) => {
        impl<$lifetime> $op for &$scheme {
            type Output = $output_type;
            fn $op_fn_name(self) -> $scheme {
                let mut clone = self.clone();
                clone.$fn_name().unwrap();
                clone
            }
        }
        
        impl<$lifetime> $op for $scheme {
            type Output = $output_type;
            fn $op_fn_name(self) -> $scheme {
                (&self).$op_fn_name()
            }
        }
    };
    (
        $lifetime: lifetime, $scheme: ty, $output_type: ty,
        $op_fn_name: ident, $op_assign_fn_name: ident, $op_fn_name_c: ident, $op_assign_fn_name_c: ident,
        $other_type: ty, $error_type: ty, $value_accessor: ident, $scheme_accessor: ident, $cutoff_type: ident,
        $op: ident, $op_assign: ident
    ) => {
        impl<$lifetime> $scheme {
            pub fn $op_assign_fn_name(&mut self, other: &$other_type) -> Result<(), $error_type> {
                self.$value_accessor.$op_assign_fn_name_c(other, self.$scheme_accessor.$cutoff_type)
            }

            pub fn $op_fn_name(&self, other: &$other_type) -> Result<$output_type, $error_type> {
                let mut clone = self.clone();
                clone.$op_assign_fn_name(other)?;
                Ok(clone)
            }
        }

        impl<$lifetime> $op<&$other_type> for &$scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: &$other_type) -> $scheme {
                <$scheme>::$op_fn_name(&self, other).unwrap()
            }
        }

        impl<$lifetime> $op<$other_type> for &$scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: $other_type) -> $scheme {
                <$scheme>::$op_fn_name(&self, &other).unwrap()
            }
        }

        impl<$lifetime> $op<&$other_type> for $scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: &$other_type) -> $scheme {
                <$scheme>::$op_fn_name(&self, &other).unwrap()
            }
        }

        impl<$lifetime> $op<$other_type> for $scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: $other_type) -> $scheme {
                <$scheme>::$op_fn_name(&self, &other).unwrap()
            }
        }

        impl<$lifetime> $op_assign<&$other_type> for $scheme {
            fn $op_assign_fn_name(&mut self, other: &$other_type) {
                <$scheme>::$op_assign_fn_name(self, other).unwrap()
            }
        }

        impl<$lifetime> $op_assign<$other_type> for $scheme {
            fn $op_assign_fn_name(&mut self, other: $other_type) {
                <$scheme>::$op_assign_fn_name(self, &other).unwrap()
            }
        }
    };
    (
        $lifetime: lifetime, $scheme: ty, $output_type: ty,
        $op_fn_name: ident, $op_assign_fn_name: ident, $op_fn_name_c: ident, $op_assign_fn_name_c: ident, $op_fn_name_s: ident, $op_assign_fn_name_s: ident,
        $other_type: ty, $error_type: ty, $value_accessor: ident, $scheme_accessor: ident, $cutoff_type: ident,
        $op: ident, $op_assign: ident
    ) => {
        scheme_op!(
            $lifetime, $scheme, $output_type,
            $op_fn_name, $op_assign_fn_name, $op_fn_name_c, $op_assign_fn_name_c,
            $other_type, $error_type, $value_accessor, $scheme_accessor, $cutoff_type,
            $op, $op_assign
        );
        impl<$lifetime> $scheme {
            pub fn $op_assign_fn_name_s(&mut self, other: &$scheme) -> Result<(), $error_type> {
                self.$op_assign_fn_name(&other.$value_accessor)
            }

            pub fn $op_fn_name_s(&self, other: &$scheme) -> Result<$output_type, $error_type> {
                self.$op_fn_name(&other.$value_accessor)
            }
        }

        impl<$lifetime> $op<&$scheme> for &$scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: &$scheme) -> $scheme {
                <$scheme>::$op_fn_name(&self, &other.$value_accessor).unwrap()
            }
        }

        impl<$lifetime> $op<$scheme> for &$scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: $scheme) -> $scheme {
                <$scheme>::$op_fn_name(&self, &other.$value_accessor).unwrap()
            }
        }

        impl<$lifetime> $op<&$scheme> for $scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: &$scheme) -> $scheme {
                <$scheme>::$op_fn_name(&self, &other.$value_accessor).unwrap()
            }
        }

        impl<$lifetime> $op<$scheme> for $scheme {
            type Output = $output_type;
            fn $op_fn_name(self, other: $scheme) -> $scheme {
                <$scheme>::$op_fn_name(&self, &other.$value_accessor).unwrap()
            }
        }

        impl<$lifetime> $op_assign<&$scheme> for $scheme {
            fn $op_assign_fn_name(&mut self, other: &$scheme) {
                self.$op_assign_fn_name_s(other).unwrap();
            }
        }

        impl<$lifetime> $op_assign<$scheme> for $scheme {
            fn $op_assign_fn_name(&mut self, other: $scheme) {
                self.$op_assign_fn_name_s(&other).unwrap();
            }
        }
    }
}
