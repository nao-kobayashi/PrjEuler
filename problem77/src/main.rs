use std::time::Instant;

fn main() {
    let start = Instant::now();
    let target = 75;
    let primes = (0..target).filter(|p| is_prime(*p)).map(|p| p).collect::<Vec<i32>>();

    for i in 1..target {
        let mut count = 0;
        calc(&primes, 0, 0, i, &mut count);
        println!("{} = count:{}", i, count);
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

fn calc(primes: &[i32], index: usize, sum: i32, target: i32, count: &mut i32) {
    let n = primes[index];
    let prv_val = sum;

    for i in 0..target / n + 1 {
        let val = (i * n) + prv_val;

        if val > target { break; }
        if val == target {
            *count += 1;
        } else {
            if index + 1 < primes.len() {
                calc(primes, index + 1, val, target, count);
            }
        }
    }
}

//素数判定
fn is_prime(n: i32) -> bool {
    if n < 2  { return false; }
    else if n == 2 { return true; }
    else if n % 2 == 0 { return false; }

    let sqrt = (n as f64).sqrt() as i32;
    (3..sqrt + 1)
        .filter(|x| x % 2 != 0)
        .all(|x|
            if n % x == 0 {
                false
            } else {
                true
            }
        )
}
