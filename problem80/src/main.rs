use std::time::Instant;

extern crate num_bigint;
use num_bigint:: BigInt;
extern crate num_traits;
use num_traits::pow::Pow;

fn main() {
    let start = Instant::now();
    let mut ans = 0;
    
    for n in 1..101 {
        let f = (n as f64).sqrt();
        if f as i32 * f as i32 != n  {
            let root = calc_root(n, 100);
            ans += root.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum::<u32>();
        }
    }
    
    println!("{}", ans);
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

fn calc_root(n: i32, digits: i32) -> BigInt {
    let mut limit = BigInt::from(10);
    limit = limit.pow((digits + 1) as u32);
    let mut a = BigInt::from(5 * n);
    let mut b = BigInt::from(5);

    while b < limit {
        if a >= b {
            a -= b.clone();
            b += 10;
        } else {
            a *= 100;
            b = (b / 10) * 100 + 5;
        }
    }

    b / 100
}
