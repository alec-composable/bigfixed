use crate::{digit::*, Index, Cutoff, CutsOff};

use std::{fmt, ops as stdops, iter::{repeat}, cmp::{max, min}};

pub mod ops;
pub mod convert;
pub mod ops_c;

#[derive(Clone, Debug)]
pub struct BigFixed {
    pub head: Digit,
    pub body: Vec<Digit>,
    pub position: Index
}

impl BigFixed {
    // remove redundant data
    pub fn format(&mut self) {
        let mut high = self.body.len();
        while high > 0 && self.body[high - 1] == self.head {
            high -= 1;
        }
        self.body.truncate(high);
        if self.body.len() > 0 {
            let mut low = 0;
            while self.body[low] == 0 {
                low += 1;
            }
            self.body.drain(0..low);
            self.position += low;
        }
        // special case: zero
        if self.head == 0 && self.body.len() == 0 {
            self.position = Index::ZERO;
        }
    }

    pub fn format_c(&mut self, cutoff: Cutoff) {
        self.format();
        if self.body.len() == 0 && !self.is_zero() {
            match cutoff.fixed {
                None => {},
                Some(x) => if self.position < x {
                    self.position = x;
                }
            }
        }
    }

    pub fn construct(head: Digit, body: Vec<Digit>, position: Index) -> BigFixed {
        let mut returner = BigFixed {
            head,
            body,
            position
        };
        returner.format();
        returner
    }

    // Restructure if necessary so that all positions in low..high are valid. Breaks format so reformat afterwards. Returns whether restructuring was necessary.
    pub fn ensure_valid_range(&mut self, low: Index, high: Index) -> bool {
        if low >= high {
            if low == high {
                return false;
            } else {
                return self.ensure_valid_range(high, low);
            }
        }
        let shifted_low = low - self.position;
        let mut reserve = Index::ZERO;
        // splice and resize
        if shifted_low < 0isize {
            reserve = -shifted_low;
            self.position = Index::from(low);
        }
        let shifted_high = high - self.position;
        let body_len = self.body.len() as isize;
        if shifted_high > body_len {
            reserve += shifted_high - body_len;
        }
        if reserve > 0isize {
            self.body.reserve(usize::from(reserve));
            if shifted_low < 0isize {
                self.body.splice(0..0, repeat(0).take(usize::from(-shifted_low)));
            }
            if shifted_high > body_len {
                self.body.resize(usize::from(shifted_high), self.head);
            }
            true
        } else {
            false
        }
    }

    // same as ensure_valid_range where range is position..=position
    pub fn ensure_valid_position(&mut self, position: Index) -> bool {
        self.ensure_valid_range(position, position + 1isize)
    }

    pub fn is_neg(&self) -> bool {
        self.head == ALLONES
    }

    // the least position which is outside of the range contained in body
    pub fn body_high(&self) -> Index {
        self.position + self.body.len()
    }

    pub fn valid_range(&self) -> stdops::Range<Index> {
        self.position..self.body_high()
    }

    pub fn int(&self) -> BigFixed {
        BigFixed::construct(
            self.head,
            self.body[(-self.position).saturating_unsigned()..self.body.len()].to_vec(),
            self.position.saturating_nonnegative()
        )
    }

    pub fn frac(&self) -> BigFixed {
        BigFixed::construct(
            0,
            self.body[0..(-self.position).saturating_unsigned()].to_vec(),
            self.position // if position is positive then body must be empty and format() resets position to 0
        )
    }

    pub fn overwrite(&mut self, src: &BigFixed) {
        self.head = src.head;
        self.body.splice(0..self.body.len(), src.body.iter().map(|x| *x));
        self.position = src.position;
    }

    pub fn shift(mut self, shift: isize) -> BigFixed {
        if !self.is_zero() {
            self.position += shift;
        }
        self
    }

    // does not require proper formatting -- fully checks if self is zero
    pub fn is_zero(&self) -> bool {
        self.head == 0 && self.body.iter().all(|&x| x == 0)
    }

    pub fn cutoff_position(&self, cutoff: Cutoff) -> Index {
        match (cutoff.fixed, cutoff.floating) {
            (None, None) => self.position, // no cutoff
            (Some(fixed), None) => max(self.position, fixed),
            (None, Some(floating)) => max(self.position, self.body_high() - floating),
            (Some(fixed), Some(floating)) => min(
                max(self.position, fixed),
                max(self.position, self.body_high() - floating)
            )
        }
    }

    // returns isize of bit position, not Digit position, so will overflow with large positions
    pub fn greatest_bit_position(&self) -> isize {
        // zero is special, just return 0
        if self.is_zero() {
            return 0;
        }
        let position = self.body_high();
        let coefficient: Digit = self[position - 1isize] ^ self.head; // greatest bit which differs from head is greatest bit here
        isize::from(position*DIGITBITS) - (coefficient.leading_zeros() + 1) as isize
    }

    pub fn abs(&self) -> BigFixed {
        if self.is_neg() {
            -self.clone()
        } else {
            self.clone()
        }
    }

    pub const ZERO: BigFixed = BigFixed {
        head: 0,
        body: vec![],
        position: Index::ZERO
    };
}

impl CutsOff for BigFixed {
    fn cutoff(&mut self, cutoff: Cutoff) {
        let amount = min(
            (self.cutoff_position(cutoff) - self.position).saturating_unsigned(),
            self.body.len()
        );
        if amount > 0 {
            self.body.drain(0..amount);
            self.position += amount;
        }
        self.format_c(cutoff);
    }
}

/*impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut body_rev = self.body.clone();
        body_rev.reverse();
        write!(f, "{} {:?} position {}", self.head, body_rev, self.position)
    }
}*/

impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (mut digits, mut point) = self.to_digits_10();
        point = digits.len() - point;
        digits.reverse();
        if point == 0isize {
            write!(f, "0").ok();
        }
        for d in digits {
            if point == 0isize {
                write!(f, ".").ok();
            }
            point -= 1isize;
            write!(f, "{}", d).ok();
        }
        write!(f, "")
    }
}

impl fmt::Binary for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut body_rev = self.body.clone();
        body_rev.reverse();
        write!(f, "{} [", self.head).ok();
        let mut first = true;
        for x in body_rev {
            if first {
                first = false;
            } else {
                write!(f, ", ").ok();
            }
            write!(f, binary_formatter!(), x).ok();
        }
        write!(f, "] position {}", self.position)
    }
}
