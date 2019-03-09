use std::time::Instant;
use std::collections::HashSet;

fn problem31(coins: &[i32], index: usize, sum: i32, count: &mut i32) {
    let n = coins[index];
    let prv_val = sum;

    for i in 0..(200 / n + 1) {
        let val = (i * n) + prv_val;

        if val == 200 {
            *count += 1;
        } else {
            if index + 1 < coins.len() {
                problem31(coins, index + 1, val, count);
            }
        }
    }
}

fn erase_num(hash: &mut HashSet<u32>, expected: i32) -> bool {
    let nums = expected.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    for n in nums {
        if hash.contains(&n) {
            hash.remove(&n);
        } else {
            return false;
        }
    }

    true
}

fn problem32() {
    let mut hash = HashSet::new();
    (1..10).for_each(|n| { hash.insert(n); });

    //多分、2桁まで * 4桁まで の組み合わせしか条件を満たさないはず。
    let mut answers = HashSet::new();
    for i in 2..100 {
        let mut hash_i = hash.clone();
        if erase_num(&mut hash_i, i) {
            for h in 2..10000 {
                let mut hash_h = hash_i.clone();
                if erase_num(&mut hash_h, h) {
                    let z = i * h;
                    if z.to_string().len() == hash_h.len() && erase_num(&mut hash_h, z) {
                        if !answers.contains(&z) {
                            answers.insert(z);
                            println!("{} * {} = {}", i, h, z);
                        }
                    }
                }
            }
        }
    }

    //println!("{:?}", answers);
    println!("{}", answers.iter().map(|n| *n).sum::<i32>());
}

fn erase_num2(i: i32, h: i32, z: i32) -> (bool, i32, i32) {
    let cnv_i = i.to_string().replace(&z.to_string(), "").parse::<i32>().unwrap_or(i);
    if cnv_i != i {
        let cnv_h = h.to_string().replace(&z.to_string(), "").parse::<i32>().unwrap_or(h);
        if cnv_h != h {
            return (true, cnv_i, cnv_h);
        }
    }
    (false, i, h)
}

fn problem33() {
    let mut ans = (1, 1);
    for i in 11..100 {
        if i % 10 == 0 { continue; }
        for h in (i + 1)..100 {
            if h % 10 == 0 { continue; }

            let z = i as f64 / h as f64 ;
            let mut numbers = i.to_string().chars()
                .chain(h.to_string().chars())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            numbers.dedup();

            for n in numbers {
                let (b, cnv_i, cnv_h) = erase_num2(i, h, n as i32);
                if b && cnv_i as f64 / cnv_h as f64  == z {
                    println!("{} {} {}\t{} {} {}", i, h, z, cnv_i, cnv_h, z);
                    ans.0 = ans.0 * cnv_i;
                    ans.1 = ans.1 * cnv_h;
                }
            }
        }
    }
    println!("ans {:?}", ans);
}

fn main() {
    let start = Instant::now();

    let mut coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    coins.sort_by_key(|n| -1 * n);
    let mut count = 0;
    problem31(&coins, 0, 0, &mut count);
    println!("count:{}", count);

    problem32();

    problem33();

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
