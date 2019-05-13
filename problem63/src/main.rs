use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut n = 1;
    let mut count = 0;
    loop {
        let mut power = 1;
        let prv_count = count;

        loop {
            let calc = calc_power(n, power);
            if calc.to_string().len() == (power as usize) {
                count += 1;
            } else {
                break;
            }
            power += 1;
        }

        if count == prv_count { break; }
        n += 1;
    }

    println!("{}", count);

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

fn calc_power(n: u128, power: u32) -> u128 {
    if power == 1 { return n; }

    let mut ans: u128 = 1;
    for _p in (0..power) {
        ans *= n;
    }
    ans
}