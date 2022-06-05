use crate::{/*digit::*,*/ Index, BigFixed, BigFixedError};

impl BigFixed {
    // ceiling(log2(self)) -- actually floor + 1
    pub fn pow2_bound(&self) -> Result<BigFixed, BigFixedError> {
        Ok(BigFixed::from((self.greatest_bit_position()? + Index::Bit(1))?.bit_value()?))
    }

    pub fn taylor_exp(&self, precision: Index) -> Result<BigFixed, BigFixedError> {
        println!("exp {} to {}", self, precision);
        // go until x^n/n! < s where x = self, s = 2^precision
        // x^n < n!*s
        let mut n = BigFixed::from(1);
        let mut xn = self.clone();
        let mut nfs = BigFixed::from(1).shift(precision)?;
        let mut powten: i128 = 1;
        let mut p = 0;
        while xn > nfs {
            powten -= 1;
            if powten == 0 {
                p += 1;
                powten = 1;
                for _i in 0..p {
                    powten *= 10;
                }
                println!("{}", powten);
            }
            n.increment()?;
            xn *= self;
            nfs *= &n;
        }
        println!("found n {}", n);
        Ok(BigFixed::from(0))
    }
}
