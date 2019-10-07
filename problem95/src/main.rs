use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let start = Instant::now();
    let mut max_len = 0;
    let mut min_elm = 0;

    // println!("{}", divisors_sum(100, 100));
    // println!("{}", divisors_sum(117, 117));
    // println!("{}", divisors_sum(65, 65));

    for i in 2..1000000 {
        let mut sums = HashSet::new();
        let mut elm = i;
        sums.insert(i);

        loop {
            let dsum = divisors_sum(elm, elm);

            if i == dsum {
                break;
            }
            
            if dsum > 1000000 || sums.contains(&dsum) {
                sums.clear();
                break;
            }

            sums.insert(dsum);
            elm = dsum;
        }

        if max_len < sums.len() as i32 {
            println!("{:?}", sums);
            max_len = sums.len() as i32;
            min_elm = sums.iter().map(|n| *n).min().unwrap();
        }
    }

    println!("{}", max_len);
    println!("{:?}", min_elm);
    let elapsed = start.elapsed();
    println!("{} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

}


//約数の合計を返す。
//220 => 素因数分解 2^2, 5, 11
//(1 + 2 + 2^2) * (1 + 5) * (1 + 11)
//= 504 = 504 - 220 = 284
//504が本来の約数の合計、284は自分自身を引いた約数の合計
fn divisors_sum(n: i32, minus: i32) -> i32 {
    let a = factoring(n as f64)
        .iter()
        .map(|(f, r)| (1..r + 1).map(|i| (*f as f64).powi(i) as i64).sum::<i64>() + 1)
        .collect::<Vec<i64>>();

    //println!("{:?}", a);
    let b = a.iter().fold(1_i64, |x, y| x * y);
    //minus own.
    b as i32 - minus
}

//素因数分解
fn factoring(n: f64) -> Vec<(i64, i32)> {
    if n == 1.0 {
        return vec![(1, 1)];
    }

    let s = n.sqrt() as usize;
    let mut y = n as i64;
    let mut r = 0;

    let mut result = Vec::new();

    for x in 2..s + 1 {
        if y % (x as i64) == 0 {
            r = 0;
            while y % (x as i64) == 0 {
                r += 1;
                y = y / x as i64;
            }
            result.push((x as i64, r));
        }
    }

    if y as usize > s {
        result.push((y as i64, 1));
    }

    result
}