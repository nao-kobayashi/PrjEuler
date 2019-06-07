use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut hash = HashMap::new();
    hash.insert(0_u32, 1_u32);
    for n in 1..10 {
        let fact = factorial(n);
        hash.insert(n, fact);
    }

    let mut ans = 0;
    for n in 1..1000000 {
        let mut val = n;
        let mut set = HashSet::new();
        set.insert(n);
        loop {
            val = factorial2(val, &hash);
            if set.contains(&val) {
                //println!("{}", set.len());
                if set.len() == 60 {
                    ans += 1;
                }

                break;
            } else {
                set.insert(val);
            }
        }
    }

    println!("ans : {}", ans);
}

fn factorial2(n: u32, cache: &HashMap<u32, u32>) -> u32 {
    n.to_string()
        .chars()
        .map(|c| cache.get(&c.to_digit(10).unwrap()).unwrap())
        .sum()
}

fn factorial(n: u32) -> u32 {
    if n == 0 { return 1; }
    (1..n + 1).fold(1, |n1, n2| n1 as u32 * n2 as u32)
}