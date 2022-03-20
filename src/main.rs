use numbers::BigFixed;

pub fn main() {
    let a = BigFixed::from((-1 as i64) as u64);
    let b = BigFixed::from((1 as i64) as u64);
    let c = a.clone() + b.clone();

    println!("{}+{}={}", a, b, c);
}
