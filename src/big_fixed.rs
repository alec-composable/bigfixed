use std::{fmt, convert, ops};

use std::cmp::{max, min};

// all zeros
pub const Z64: u64 = 0;
// all ones
pub const O64: u64 = (-1 as i64) as u64;
pub const O64D: u128 = O64 as u128;
// highest bit only
pub const H64: u64 = (1 as u64) << 63;

#[derive(Clone)]
pub struct BigFixed {
    pub data: Vec<u64>,
    pub data_low: i64,
    pub data_high: i64,
}

impl BigFixed {
    pub fn neg(&self) -> bool {
        self.data[self.width() - 1] >= H64
    }

    pub fn width(&self) -> usize {
        (self.data_high - self.data_low) as usize
    }

    pub fn get(&self, position: i64) -> u64 {
        if position < self.data_low {
            0
        } else if position > self.data_high {
            if self.neg() {
                O64
            } else {
                0
            }
        } else {
            self.data[(position - self.data_low) as usize]
        }

    }

    pub fn raw_set(&mut self, position: i64, value: u64) {
        self.data[(position - self.data_low) as usize] = value
    }

    pub fn set(&mut self, position: i64, value: u64) {
        if position > self.data_high {

        } else if position < self.data_low {

        }
        self.raw_set(position, value)
    }
}

impl fmt::Display for BigFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl convert::From<i64> for BigFixed {
    fn from(i: i64) -> BigFixed {
        BigFixed {
            data: vec![i as u64],
            data_low: 0,
            data_high: 0
        }
    }
}

impl convert::From<i32> for BigFixed {
    fn from(i: i32) -> BigFixed {
        BigFixed::from(i as i64)
    }
}

impl convert::From<u64> for BigFixed {
    fn from(n: u64) -> BigFixed {
        BigFixed {
            data: vec![n,0],
            data_low: 0,
            data_high: 1
        }
    }
}

impl convert::From<u32> for BigFixed {
    fn from(n: u32) -> BigFixed {
        BigFixed::from(n as u64)
    }
}

impl ops::Add for BigFixed {
    type Output = BigFixed;
    fn add(self, other: BigFixed) -> BigFixed {
        &self + &other
    }
}

impl ops::Add for &BigFixed {
    type Output = BigFixed;
    fn add(self, other: &BigFixed) -> BigFixed {
        let low = min(self.data_low, other.data_low);
        let high = max(self.data_high, other.data_high);
        let width = high - low;
        // leave room for one overflow
        let mut v = vec![0 as u64; (width + 1) as usize];
        let mut carried: u128 = 0;
        let mut res: u128;
        for i in low..high {
            res = (self.get(i) as u128) + (other.get(i) as u128) + carried;
            carried = res >> 64;
            v[(i-low) as usize] = (res & O64D) as u64;
        }
        v[width as usize] = (carried & O64D) as u64;
        BigFixed {
            data: v,
            data_high: high,
            data_low: low
        }
    }
}
