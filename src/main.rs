use astro_float::{BigFloat, RoundingMode};

fn main() {
    let n = 2000000;
    let p = (n / (n as f32).log2() as usize).max(64);
    let one = BigFloat::from_i32(1, p);
    let two = BigFloat::from_i32(2, p);
    let rm = RoundingMode::Up;
    let sqrt_5 = BigFloat::from_i32(5, p).sqrt(n,rm);

    let inner_one = BigFloat::from_i32(1, p).add(&sqrt_5, p, rm).div(&two, p, rm).powi(n, p, rm);
    let inner_two = BigFloat::from_i32(1, p).sub(&sqrt_5, p, rm).div(&two, p, rm).powi(n, p, rm);

    let fib: BigFloat = one.div(&sqrt_5, p, rm).mul(&inner_one.sub(&inner_two, p, rm), p, rm).int();

    println!("{}",fib);
}