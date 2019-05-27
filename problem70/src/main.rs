fn main() {
    let mut smaller_n = 999999999_u32;
    let mut smaller: f64 = 999999999999.0;
    let mut primes = Vec::new();
    for n in 2..30000 {
        if is_prime(n) {
            primes.push(n);
        }
    }

    for (i, p1) in primes.iter().enumerate() {
        for index in (i + 1)..primes.len() {
            let p2 = primes[index];
            let n = p1 * p2;
            if n <= 10000000 {
                let ans = (n as f64) * (1.0 - (1.0 / *p1 as f64)) * (1.0 - (1.0 / p2 as f64));

                let a = n.to_string();
                let b = ans.to_string();
                if a.len() == b.len() && is_same_str(&a, &b) {
                    if smaller > (n as f64) / ans {
                        smaller_n = n as u32;
                        smaller = (n as f64) / ans;
                    }
                }
            }
        }
    }
    println!("answer:{}({})", smaller_n, smaller);
}

fn is_same_str(a: &String, b: &String) -> bool {
    let mut a_v = a.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    
    a_v.sort(); 

    let mut b_v = b.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    
    b_v.sort();

    let s1 = a_v.join("");
    let s2 = b_v.join("");
    //println!("{} {}", s1, s2);
    return s1 == s2;
}

//素数判定
fn is_prime(n: i32) -> bool {
    if n < 2  { return false; }
    else if n == 2 { return true; }
    else if n % 2 == 0 { return false; }

    let sqrt = (n as f64).sqrt() as i32;
    (3..sqrt + 1)
        .filter(|x| x % 2 != 0)
        .all(|x|
            if n % x == 0 {
                false
            } else {
                true
            }
        )
}
