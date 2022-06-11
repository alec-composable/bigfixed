use bigfixed::*;
//use paste::paste;

pub fn main() {
    let scheme = schemes::SCHEME_F64;
    let n = big_fixed_from!(scheme, 1);
    let d = BigFixed::from(9);
    println!("{:}", n / d);
}
