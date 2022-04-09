use crate::digit::*;

use std::{convert, ops::{Index, IndexMut}, fmt};

#[derive(Clone)]
pub struct Tail {
    pub data: Vec<Digit>
}

impl Tail {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn collapse(&mut self) -> bool {
        let len = self.data.len();
        'sub: for sub in 1..=(len / 2) {
            if len % sub != 0 {
                continue;
            }
            let copies = len / sub;
            for i in 0..sub {
                let test = self.data[i];
                for copy in 1..copies {
                    if self.data[sub * copy + i] != test {
                        continue 'sub;
                    }
                }
            }
            self.data.resize(sub, 0);
            return true;
        }
        false
    }

    pub fn shift(&mut self, up: isize) {
        let len = self.data.len();
        if up >= 0 {
            self.data.rotate_right((up as usize) % len);
        } else {
            self.data.rotate_left(((-up) as usize) % len);
        }
    }

    pub fn resize(&mut self, newlen: usize) {
        let oldlen = self.data.len();
        self.data.resize(newlen, 0);
        if oldlen < newlen {
            // probably a fancier iterator way to do this but oh well
            for i in oldlen..newlen {
                self.data[i] = self.data[i % oldlen];
            }
        }
    }

    pub fn zero() -> Tail {
        Tail::from(vec![0])
    }
}

pub struct TailIterator<'a> {
    tail: &'a Tail,
    on: usize
}

impl<'a> Iterator for TailIterator<'a> {
    type Item = Digit;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.tail[self.on].clone();
        self.on += 1;
        if self.on == self.tail.len() {
            self.on = 0;
        }
        Some(val)
    }
}

impl<'a> IntoIterator for &'a Tail {
    type Item = Digit;
    type IntoIter = TailIterator<'a>;
    fn into_iter(self) -> TailIterator<'a> {
        TailIterator {
            tail: self,
            on: 0
        }
    }
}

impl convert::From<Vec<Digit>> for Tail {
    fn from(data: Vec<Digit>) -> Tail {
        Tail {
            data
        }
    }
}

macro_rules! digit_array {
    ($arr: expr) => {
        $arr.iter().map(|x| *x as Digit).collect::<Vec<Digit>>()
    };
}

macro_rules! make_convert {
    ($type: ty) => {
        impl convert::From<&[$type]> for Tail {
            fn from(data: &[$type]) -> Tail {
                Tail::from(digit_array!(data))
            }
        }
    };
}

make_convert!(usize);
make_convert!(isize);
make_convert!(u8);
make_convert!(i8);
make_convert!(u16);
make_convert!(i16);
make_convert!(u32);
make_convert!(i32);
make_convert!(u64);
make_convert!(i64);
make_convert!(u128);
make_convert!(i128);

macro_rules! index_and_mut {
    ($type: ty) => {
        impl Index<$type> for Tail {
            type Output = Digit;
            fn index(&self, index: $type) -> &Digit {
                &self.data[index.rem_euclid(self.data.len() as $type) as usize]
            }
        }

        impl IndexMut<$type> for Tail {
            fn index_mut(&mut self, index: $type) -> &mut Digit {
                self.data.index_mut(index.rem_euclid(self.data.len() as $type) as usize)
            }
        }
    };
}

index_and_mut!(usize);
index_and_mut!(isize);
index_and_mut!(u8);
index_and_mut!(i8);
index_and_mut!(u16);
index_and_mut!(i16);
index_and_mut!(u32);
index_and_mut!(i32);
index_and_mut!(u64);
index_and_mut!(i64);
index_and_mut!(u128);
index_and_mut!(i128);

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
