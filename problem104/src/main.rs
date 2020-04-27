use std::cmp::max;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut a = vec![1];
    let mut b = vec![1];
    let mut c = vec![0];
    let mut count = 2;

    while count < 1000000 {
        let mut carry = 0;
        let index = max(a.len(), b.len());
        c.clear();

        for i in 0..index {
            let value1 = if a.len() > i {
                a[i]
            } else {
                0
            };

            let value2 = if b.len() > i {
                b[i]
            } else {
                0
            };

            let sum = value1 + value2 + carry;
            let real_value = if sum >= 10 {
                carry = 1;
                sum - 10
            } else {
                carry = 0;
                sum
            };

            c.push(real_value);
        }

        if carry > 0 {
            c.push(carry);
        }

        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut b, &mut c);
        count += 1;

        if count % 1000 == 0 {
            println!("{}", count);
        }

        if b.len() >= 10 {
            //reverseしている
            if check(&b[0..9]) && check(&b[b.len() - 9..]) {
                println!("index => {}", count);
                break;
            }
        }
    }

    let elapsed = start.elapsed();
    println!(
        "{} ms",
        (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
    );
}

fn check(source: &[i32]) -> bool {
    let mut bits = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for n in source {
        if n - 1 >= 0 {
            bits[(n - 1) as usize] = 1;
        }
    }

    bits.iter().all(|bit| *bit == 1)
}
