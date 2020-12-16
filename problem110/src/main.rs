extern crate num_bigint;
use std::collections::BTreeMap;
use num_bigint::BigInt;
use std::time::Instant;

/*
n = 1260

n^2 = 1587600
2^4 * 3^4 * 5^2 * 7^2 = n^2

divisors
(4 + 1)(4 + 1)(2 + 1)(2+ 1)
5 * 5 * 3 * 3 = 225
225 / 2 = 112.5 = 112
112 + 1 = 113 cases
*/
fn main() {
    let start_time = Instant::now();
    //let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let mut primes = Vec::new();
    primes.push(BigInt::from(2));
    primes.push(BigInt::from(3));
    primes.push(BigInt::from(5));
    primes.push(BigInt::from(7));
    primes.push(BigInt::from(11));
    primes.push(BigInt::from(13));
    primes.push(BigInt::from(17));
    primes.push(BigInt::from(19));
    primes.push(BigInt::from(23));
    primes.push(BigInt::from(29));
    primes.push(BigInt::from(31));
    primes.push(BigInt::from(37));
    let limit = 4000000;

    let mut todo = BTreeMap::new();
    todo.insert(BigInt::from(1), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    loop {
        let tuple = todo.iter().next().unwrap();
        let (mut value, mut exponents) = (tuple.0.clone(), tuple.1.clone());
        todo.remove(&value);
        //println!("{}", &value);
        //println!("{:?}", &exponents);

        let mut unique_factors = 1;
        for x in &exponents {
            unique_factors *= 2 * *x + 1;
        }
        unique_factors += 1;
        unique_factors /= 2;

        if unique_factors >= limit {
            println!("{}", value);
            break;
        }

        for i in 0..exponents.len() {
            //optimizing
            if exponents[i] == 1 && i > 3 {
                break;
            }

            *exponents.get_mut(i).unwrap() += 1;
            value *= &primes[i];
            todo.insert(value.clone(), exponents.clone());
        }
    }

    let elapsed = start_time.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
