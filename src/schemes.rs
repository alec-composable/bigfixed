use crate::{Index, Cutoff, Rounding, CutoffScheme};

use paste::paste;

#[macro_export]
macro_rules! float_scheme {
    ($arithmetic: expr, $comparisons: expr) => {
        paste!{
            pub const [<FLOAT_ $arithmetic _ $comparisons>]: CutoffScheme = CutoffScheme {
                arithmetic: Cutoff {
                    fixed: None,
                    floating: Some(Index::Bit($arithmetic)),
                    round: Rounding::Floor
                },
                comparisons: Cutoff {
                    fixed: None,
                    floating: Some(Index::Bit($comparisons)),
                    round: Rounding::Round
                }
            };
        }
    };
}

// These approximate the behavior of native floats. Arithmetic is a little wider than the native type allows and 

float_scheme!(32, 22);
pub const SCHEME_F32: CutoffScheme = FLOAT_32_22;

float_scheme!(64, 51);
pub const SCHEME_F64: CutoffScheme = FLOAT_64_51;

float_scheme!(80, 64);
