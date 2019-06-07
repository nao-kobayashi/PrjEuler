use std::cmp::{ min, max };

/*
a = m^​2 ​​− n​^2​​
b = 2mn
c = m^2 + n^2
*/
fn main() {
    let limit = 1500000_u128;
    let mut answer = (0..1500001).map(|_n| 0).collect::<Vec<i32>>();

    for n1 in 2..(limit as f64).sqrt() as u128 + 1  {
        for n2 in 1..n1 {
            if (n1 + n2) % 2 != 1 || gcd(n1, n2) != 1 {
                continue;
            }

            let m = max(n1, n2);
            let n = min(n1, n2);
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            
            if a + b + c <= limit {
                let index = (a + b + c) as usize;
                answer[index] += 1;

                let mut mul = 2;
                loop {
                    let aa = a * mul;
                    let bb = b * mul;
                    let cc = c * mul;

                    let index = aa + bb + cc;
                    if index <= limit {
                        answer[index as usize] += 1;
                    } else {
                        break;
                    }
                    mul += 1;
                }
            }
        }
    }

    let ans = answer.iter().filter(|n| **n == 1).count();
    println!("ans {:?}", ans);
}



fn gcd(a: u128, b: u128) -> u128 {
    let mut max_ = max(a, b);
    let mut min_ = min(a, b);

    loop {
        if min_ == 0 { return 0; }

        if max_ % min_ == 0 {
            return min_;
        } else {
            let aa = max(min_, max_ % min_);
            let bb = min(min_, max_ % min_);
            max_ = aa;
            min_ = bb;
        }
    }
}
