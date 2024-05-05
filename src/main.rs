use astro_float::{BigFloat, RoundingMode};
use::std::env;
fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("no argument");
        return;
    }

    let n:usize = args[1].trim().parse::<usize>().unwrap_or(1);

    let p: usize = ((n as f64 * (1.62f64).log2()) as usize).max(32); //needed precision is log_2(golden_ratio^n)
    let one: BigFloat = BigFloat::from_i32(1, p); //precalc one
    let two: BigFloat = BigFloat::from_i32(2, p); //precalc two
    let rm: RoundingMode = RoundingMode::Up;
    let sqrt_5: BigFloat = BigFloat::from_i32(5, p).sqrt(p,rm); //precalc sqrt(5)

    let inner_one: BigFloat = BigFloat::from_i32(1, p).add(&sqrt_5, p, rm).div(&two, p, rm).powi(n, p, rm);
    let inner_two: BigFloat = BigFloat::from_i32(1, p).sub(&sqrt_5, p, rm).div(&two, p, rm).powi(n, p, rm);

    let fib: BigFloat = one.div(&sqrt_5, p, rm).mul(&inner_one.sub(&inner_two, p, rm), p, rm).int();

    println!("{}",fib);
}