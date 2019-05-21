extern crate num_bigint;
use num_bigint:: BigInt;

/*
x0 = 1, x1 = a0, x2 = x0 + a1 * x1
y0 = 0, y1 = 1, y2 = y0 + a1 * y1

root 7 = [2; (1, 1, 1, 4)]
x0 = 1, y0 = 0
x1 = 2, y1 = 1
x2 = 3, y2 = 1
x3 = 5, y3 = 2
x4 = 8, y4 = 3
 */
fn main() {
    let mut max = BigInt::from(0);
    //for d in 13..14 {
    for d in 2..1001 {
        let (period, nums) = get_period(d);
        if period > 0 {
            let ans = calc(period, &nums, d);
            if ans > max {
                println!("x = {}  d = {}", ans, d);
                max = ans.clone();
            }
        }
    }
}

fn calc(period: u128, nums: &Vec<u128>, d: u128) -> BigInt {
    let mut x0 = BigInt::from(1);
    let mut y0 = BigInt::from(0);
    let mut x1 = BigInt::from(nums[0]);
    let mut y1 = BigInt::from(1);
    let mut x2 = BigInt::from(0);
    let mut y2 = BigInt::from(0);

    //Maybe the value is found.
    loop {
        for i in 0..period {
            let i = (i + 1) as usize;
            x2 = x0.clone() + BigInt::from(nums[i]) * x1.clone();
            y2 = y0.clone() + BigInt::from(nums[i]) * y1.clone();
            x0 = x1.clone();
            y0 = y1.clone();
            x1 = x2.clone();
            y1 = y2.clone();

            if (x2.clone() * x2.clone()) - (BigInt::from(d) * y2.clone() * y2.clone()) == BigInt::from(1) {
                return x2;
            }
        }
    }

    return BigInt::from(0);
}

fn get_period(x: u128) -> (u128, Vec<u128>) {
    let root = (x as f64).sqrt() as u128;

    if root * root == x {
        return (0, Vec::new());
    }

    let mut a = root;
    let mut numerator: u128 = 0;
    let mut denominator: u128 = 1;

    let mut period = 0;
    let mut nums = Vec::new();
    nums.push(a);

    while a != 2 * root {
        numerator = denominator * a - numerator;
        denominator = (x - numerator * numerator) / denominator;
        a = (root + numerator) / denominator;
 
        nums.push(a);
        period += 1;
    }

    //println!("{} = {:?}   period:{}", x, nums, period);
    return (period, nums);
}