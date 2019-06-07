use std::cmp::{ max, min };

fn main() {
    let mut cnt = 0;

    for d in 2..12001 {
    //for d in 2..9 {
        //println!("{} {}", (1 * d) / 3, (1 * d) / 2);
        for n in (1 * d) / 3 - 1..(1 * d) / 2 + 1 {
            if gcd(d, n) == 1 && d * 1 > n * 2 && d * 1 < n * 3 {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}


fn gcd(a: i64, b: i64) -> i64 {
    let max = max(a, b);
    let min = min(a, b);

    if min == 0 { return 0; }

    if max % min == 0 {
        min
    } else {
        gcd(min, max % min)
    }
}
