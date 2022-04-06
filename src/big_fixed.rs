use crate::{digit::*, Tail};

use std::{fmt, /*cmp::{max, min}, iter*/};

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
        // absorb body into tail
        let mut shift = 0;
        while self.body.len() > 0 && self.body[0] == self.tail[shift] {
            self.body.remove(0);
            self.position += 1;
            shift += 1;
        }
        self.tail.shift(-shift);
        // collapse tail
        self.tail.collapse();
        // special case: bad tail
        if self.tail.len() == 1 && self.tail[0usize] == ALLONES {
            self.tail[0usize] = 0;
            self.add_digit(1, self.position);
            return;
        }
        // collapse body
        while self.body.len() > 0 && self.body[self.body.len() - 1] == self.head {
            self.body.pop();
        }
        // special case: zero
        if self.head == 0 && self.body.len() == 0 && self.tail.len() == 1 && self.tail[0usize] == 0 {
            self.position = 0;
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

    // Restructure if necessary so that position is in the body region. Breaks format so reformat afterwards. Returns whether restructuring was necessary.
    pub fn ensure_valid_position(&mut self, position: isize) -> bool {
        let shifted = position - self.position;
        if shifted >= 0 {
            let shifted = shifted as usize;
            if shifted >= self.body.len() {
                self.body.resize(shifted + 1, self.head);
                true
            } else {
                false
            }
        } else {
            let pos_shift = (-shifted) as usize;
            let len = self.tail.len();
            self.body.splice(0..0, self.tail.into_iter().skip(len - (pos_shift %  len)).take(pos_shift));
            self.tail.shift(pos_shift as isize);
            self.position = position;
            true
        }
    }

/*
    // Ensure that all positions p with low <= p < high are valid. This breaks trim format! Call trim again before finishing.
    pub fn validate_range(&mut self, low: isize, high: isize) {
        let shift = max(0, self.position - low) as usize;
        let lenu = self.data.len();
        let len = lenu as isize;
        let pad = max(0, high - (self.position + len)) as usize;
        let grow = shift + pad;
        if grow > 0 {
            self.data.reserve(grow);
        }
        if shift > 0 {
            self.data.splice(0..0, iter::repeat(0).take(shift));
            self.position -= shift as isize;
        }
        if pad > 0 {
            self.data.splice(lenu..lenu, iter::repeat(self.head_digit).take(pad));
        }
    }

    pub fn position_high(&self) -> isize {
        self.position + self.data.len() as isize
    }

    // Ensure that position is valid. Returns whether an extension was required.
    pub fn ensure_position(&mut self, position: isize) -> bool {
        println!("ensuring {} for {:?} {}", position, self.data, self.position);
        // these checks are theoretically unnecessary but they avoid trivial calls to expand_range
        if position < self.position {
            self.expand_range(position, self.position_high());
            true
        } else if position >= self.position_high() {
            self.expand_range(self.position, position);
            true
        } else {
            false
        }
    }

    pub fn set(&mut self, d: Digit, position: isize) {
        self.ensure_position(position);
        self.data[(position - self.position) as usize] = d;
    }

    pub fn greatest_digit(&self) -> Digit {
        if self.data.len() > 0 {
            self.data[self.data.len() - 1]
        } else {
            0
        }
    }

    pub fn is_neg(&self) -> bool {
        self.greatest_digit() >= GREATESTBIT
    }

    pub fn cast_unsigned(mut self) -> BigFixed {
        if self.is_neg() {
            self.data.push(0);
        }
        self
    }

    pub fn cast_signed(mut self) -> BigFixed {
        self
    }

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

    pub fn int_data(&self) -> &[Digit] {
        &self.data[(min(max(0, -self.position) as usize, self.data.len()))..self.data.len()]
    }

    pub fn int(&self) -> BigFixed {
        BigFixed::construct(
            self.int_data().to_vec(),
            max(0, self.position),
            false
        )
    }

    pub fn frac_data(&self) -> &[Digit] {
        &self.data[0..(max(0, min(-self.position, self.data.len() as isize)) as usize)]
    }

    pub fn frac(&self) -> BigFixed {
        BigFixed::construct(
            self.frac_data().to_vec(),
            0,
            true
        )
    }

    pub fn is_zero(&self) -> bool {
        self.data.len() == 0
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
