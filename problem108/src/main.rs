/*
(1/x) + (1/y) = (1/n)
xy - xn - yn = 0

n^2 = (x-n)(y-n)
*/
fn main() {
    let mut n = 1;
    let threshold = 1000;

    loop {
        let divisors = count_divisors(n * n);
        let half = (divisors + 1) / 2;
        if half >= threshold {
            println!("n => {}", n);
            println!("k => {}", half);
            break;
        }
        n += 1;
    }
}

fn count_divisors(n: u128) -> u128 {
    let mut result = 1;
    let mut reduce = n;

    for mut divisor in 2..=reduce {
        if divisor % 2 == 0 && divisor > 2 {
            divisor += 1;
        }

        if divisor > 100 {
            break;
        }

        let mut exponent = 0;
        while reduce % divisor == 0 {
            exponent += 1;
            reduce /= divisor;
        }

        result *= exponent + 1;
    }

    result
}
