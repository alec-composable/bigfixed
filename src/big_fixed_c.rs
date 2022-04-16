use crate::{Cutoff, CutsOff, BigFixed};

use std::{fmt, convert::From};

#[derive(Clone)]
pub struct BigFixedC {
    pub src: BigFixed,
    pub cutoff: Cutoff,
}

impl From<(BigFixed, Cutoff)> for BigFixedC {
    fn from((mut src, cutoff): (BigFixed, Cutoff)) -> BigFixedC {
        src.cutoff(cutoff);
        BigFixedC {
            src,
            cutoff
        }
    }
}

impl fmt::Display for BigFixedC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}

impl fmt::Debug for BigFixedC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.src)
    }
}
