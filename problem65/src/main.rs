extern crate num_bigint;
use num_bigint:: BigInt;

fn main() {
    let mut val1 = BigInt::from(0);
    let mut val2 = BigInt::from(1);
    let mut val3 = BigInt::from(2);
    let mut fraction = 1;

    for k in 2..101 {
        if k % 3 == 0 {
            fraction = (k / 3) * 2;
        } else {
            fraction = 1;
        }

        val1 = val2.clone();
        val2 = val3.clone();

        if fraction == 1 {
            val3 = val1.clone() + val2.clone();
        } else {
            val3 = val1.clone() + val2.clone() * fraction;
        }

        //println!("{} {} {} {}", k, &val1, &val2, &val3);
    }

    //println!("{} {} {}", &val1, &val2, &val3);

    let ans = &val3.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();

    println!("ans: {}", ans);
}
