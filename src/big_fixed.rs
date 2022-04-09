use crate::{digit::*, Tail};

use std::{fmt, ops as stdops /*cmp::{max, min}, iter*/};

//pub mod convert;

pub mod convert;
pub mod ops;

#[derive(Clone)]
pub struct BigFixed {
    pub head: Digit,
    pub body: Vec<Digit>,
    pub tail: Tail,
    pub position: isize
}

impl BigFixed {
    // remove redundant data
    pub fn format(&mut self) {
        // collapse into head
        while self.body.len() > 0 && self.body[self.body.len() - 1] == self.head {
            self.body.pop();
        }
        // collapse tail
        self.tail.collapse();
        // absorb body into tail
        let mut shift = 0;
        while self.body.len() > 0 && self.body[0] == self.tail[shift] {
            self.body.remove(0);
            self.position += 1;
            shift += 1;
        }
        // head-tail interactions
        if self.body.len() == 0 {
            let tail_len = self.tail.len() as isize;
            if tail_len == 1 {
                // special case: zero
                if self.head == 0 && self.tail[0usize] == 0 {
                    self.position = 0;
                    return;
                }
            } else {
                // absorb head into tail
                while shift < tail_len && self.head == self.tail[-(shift + 1)] {
                    shift += 1;
                    self.position -= 1;
                }
            }
        }
        self.tail.shift(-shift);
        // special case: bad tail
        if self.tail.len() == 1 && self.tail[0usize] == ALLONES {
            self.tail[0usize] = 0;
            self.add_digit(1, self.position);
            // tail changed, start over
            return self.format();
        }
    }

    pub fn construct(head: Digit, body: Vec<Digit>, tail: Tail, position: isize) -> BigFixed {
        let mut returner = BigFixed {
            head,
            body,
            tail,
            position
        };
        returner.format();
        returner
    }

    // Restructure if necessary so that all positions in low..high are valid. Breaks format so reformat afterwards. Returns whether restructuring was necessary.
    pub fn ensure_valid_range(&mut self, low: isize, high: isize) -> bool {
        if low >= high {
            if low == high {
                return false;
            } else {
                return self.ensure_valid_range(high, low);
            }
        }
        let shifted_low = low - self.position;
        let shifted_high = high - self.position;
        let increase_high;
        let increase_low;
        let mut reserve = 0;
        if shifted_high > self.body.len() as isize {
                reserve = (shifted_high - self.body.len() as isize) as usize;
                increase_high = true;
        } else {
            increase_high = false;
        }
        if shifted_low < 0 {
            reserve += (-shifted_low) as usize;
            increase_low = true;
        } else {
            increase_low = false;
        }
        if reserve > 0 {
            self.body.reserve(reserve);
            if increase_low {
                let pos_shift = (-shifted_low) as usize;
                let len = self.tail.len();
                self.body.splice(0..0, self.tail.into_iter().skip(len - (pos_shift % len)).take(pos_shift));
                self.tail.shift(pos_shift as isize);
                self.position = low;
            }
            if increase_high {
                self.body.resize(shifted_high as usize, self.head);
            }
            true
        } else {
            false
        }
    }

    // same as ensure_valid_range where range is position..=position
    pub fn ensure_valid_position(&mut self, position: isize) -> bool {
        self.ensure_valid_range(position, position + 1)
    }

    pub fn is_neg(&self) -> bool {
        self.head == ALLONES
    }

    pub fn body_high(&self) -> isize {
        self.position + self.body.len() as isize
    }

    pub fn tail_low(&self) -> isize {
        self.position - self.tail.len() as isize
    }

    pub fn valid_range(&self) -> stdops::Range<isize> {
        self.position..self.body_high()
    }

/*
    pub fn neg(&self) -> BigFixed {
        let mut data: Vec<Digit> = self.data.iter().map(
            |x| x ^ ALLONES
        ).collect();
        let i = 0;
        while i < self.data.len() {
            data[i] = data[i].wrapping_add(1);
            if data[i] != 0 {break}
        }
        BigFixed::construct(data, self.position, false)
    }

    pub fn abs(&self) -> BigFixed {
        if self.is_neg() {
            self.neg()
        } else {
            self.clone()
        }
    }

    pub fn shift(&mut self, shift: isize) -> &BigFixed {
        self.position = self.position + shift;
        self
    }
    */
}


impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut body_rev = self.body.clone();
        body_rev.reverse();
        let mut tail_rev = self.tail.data.clone();
        tail_rev.reverse();
        write!(f, " {} {:?}.{:?} position {}", self.head, body_rev, tail_rev, self.position)
    }
}
