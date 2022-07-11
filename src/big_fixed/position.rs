use crate::{
    Digit,
    Index,
    IndexError,
    BigFixed,
    BigFixedError
};

impl<D: Digit> BigFixed<D> {
    // If self.position is Index::Bit this will bit shift as necessary and turn it into Index::Position.
    pub fn fix_position(&mut self) -> Result<(), BigFixedError> {
        match self.position {
            Index::Position(_) => return Ok(()),
            Index::Bit(_) => {
                let diff = self.position.bit_position_excess()?;
                self.position = self.position.cast_to_position()?;
                // b = p*DIGITBITS + diff and 0 <= diff < DIGITBITS
                // shift data left by diff
                if diff > 0 {
                    let opdiff = D::DIGITBITS - diff;
                    let keepmask: D = D::ALLONES >> diff;
                    let carrymask: D = !keepmask;
                    let len = self.body.len();
                    if len > 0 {
                        let high_digit = ((self.head & keepmask) << diff) | ((self.body[len - 1] & carrymask) >> opdiff);
                        if high_digit != self.head {
                            self.body.push(high_digit);
                        }
                        for i in (1..len).rev() {
                            self.body[i] = ((self.body[i] & keepmask) << diff) | ((self.body[i-1] & carrymask) >> opdiff);
                        }
                        self.body[0] = (self.body[0] & keepmask) << diff;
                    } else if self.is_neg() {
                        self.body.push(((D::ALLONES & keepmask) << diff).into());
                    }
                }
            },
            Index::DigitTypeInUse(_) => return Err(IndexError::UsedDigitTypeAsIndex.into())
        }
        Ok(())
    }

    pub fn properly_positioned(&self) -> bool {
        match self.position {
            Index::Position(_) => true,
            _ => false
        }
    }

    pub fn properly_positioned_result(&self) -> Result<(), BigFixedError> {
        if self.properly_positioned() {
            Ok(())
        } else {
            Err(BigFixedError::ImproperlyPositioned)
        }
    }

    pub fn trim_head(&mut self) {
        let mut count = 0;
        for x in self.body.iter().rev() {
            if *x == self.head {
                count += 1;
            } else {
                break;
            }
        }
        self.body.truncate(self.body.len() - count);
    }

    pub fn trim_tail(&mut self) -> Result<(), BigFixedError> {
        let mut low = 0;
        for x in self.body.iter() {
            if *x == D::ZERO {
                low += 1;
            } else {
                break;
            }
        }
        self.body.drain(0..low);
        self.position = (self.position + Index::Position(low as isize))?;
        Ok(())
    }

    // fix position then remove redundant body data
    pub fn format(&mut self) -> Result<(), BigFixedError> {
        if self.head != D::ZERO {
            self.head = D::ALLONES;
        }
        self.fix_position()?;
        self.trim_head();
        if self.body.len() > 0 {
            self.trim_tail()?;
        }
        // special case: zero
        if self.head == D::ZERO && self.body.len() == 0 {
            self.position = Index::Position(0);
        }
        Ok(())
    }

    pub fn construct(head: D, body: Vec<D>, position: Index<D>) -> Result<BigFixed<D>, BigFixedError> {
        let mut returner = BigFixed {
            head,
            body,
            position
        };
        returner.format()?;
        Ok(returner)
    }
}
