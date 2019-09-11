use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let start_time = Instant::now();

    // let limit = 50000000;
    let limit = 200;
    let max = (limit as f64).sqrt() as i128;
    let mut twice = Vec::new();
    let mut third = Vec::new();
    let mut fourth = Vec::new();
    let mut sums = HashSet::new();

    // println!("loop max:{}", max);

    for n in 2..max {
        if is_prime(n) {
            twice.push(n * n);
            third.push(n * n * n);
            fourth.push(n * n * n * n);
        }
    }

    for x in twice.iter() {
        for y in third.iter() {
            if x + y < limit {
                for z in fourth.iter() {
                    if x + y + z < limit {
                        // println!("{} {} {}", x, y, z);
                        sums.insert(x + y + z);
                    }
                }
            }
        }
    }

    println!("ans:{}", sums.len());
    let elapsed = start_time.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

//素数判定
fn is_prime(n: i128) -> bool {
    if n < 2  { return false; }
    else if n == 2 { return true; }
    else if n % 2 == 0 { return false; }

    let sqrt = (n as f64).sqrt() as i128;
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