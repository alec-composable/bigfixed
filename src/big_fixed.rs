use crate::{digit::*, Index};

use std::{fmt, ops as stdops, iter::{repeat}};

pub mod ops;
pub mod convert;

#[derive(Clone)]
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

    pub fn is_zero(&self) -> bool {
        self.head == 0 && self.body.iter().all(|x| *x == 0)
    }
}


impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut body_rev = self.body.clone();
        body_rev.reverse();
        write!(f, "{} {:?} position {}", self.head, body_rev, self.position)
    }
}

impl fmt::Debug for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut body_rev = self.body.clone();
        body_rev.reverse();
        write!(f, "{} {:?} position {}", self.head, body_rev, self.position)
    }
}
