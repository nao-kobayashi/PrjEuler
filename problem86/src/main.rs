use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let start_time = Instant::now();
    let mut start = 1;
    let mut m = 1500;
    let mut count = HashSet::new();
    let max = 1000000;

    'main:
    loop {
        for x in start..m + 1 {
            for y in 1..m + 1 {
                if x < y { break; }
                for z in 1..m + 1 {
                    if y < z { break; }

                    let a = y + z;
                    let len = x * x + a * a;
                    let maybe = (len as f64).sqrt();

                    if (maybe as i32 * maybe as i32) == len {
                        let mut cubic = vec![x, y, z];
                        cubic.sort();
                        count.insert(cubic);
                        if count.len() > max {
                            break 'main;
                        }
                    }
                }
            }
        }

        // println!("m:{} ans:{}", m, count.len());
        start = m;
        m += 1;
    }

    println!("m:{} ans:{}", m, count.len());
    let elapsed = start_time.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
