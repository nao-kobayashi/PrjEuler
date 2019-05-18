fn main() {
    let last = 10000;

    let mut count = 0;
    for i in 2..last + 1 {
        let period = get_period_length(i);
        if period % 2 == 1 {
            count += 1;
        }
    }

    println!("odd is {}", count);
}

fn get_period_length(x: u128) -> u128 {
    let root = (x as f64).sqrt() as u128;

    if root * root == x {
        return 0;
    }

    let mut a = root;
    let mut numerator: u128 = 0;
    let mut denominator: u128 = 1;

    let mut period = 0;

    while a != 2 * root {
        numerator = denominator * a - numerator;
        denominator = (x - numerator * numerator) / denominator;
        a = (root + numerator) / denominator;
 
        period += 1;
    }

    return period;
}