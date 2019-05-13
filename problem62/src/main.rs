use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();

    let mut n: u128 = 1;
    let mut h: HashMap<String, u128> = HashMap::new();
    let mut h_init: HashMap<String, u128> = HashMap::new();

    loop {
        let ans = n * n * n;
        let mut v = ans.to_string()
            .chars()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        v.sort();
        let key = v.join("");

        if h.contains_key(&key) {
            let num = h.get(&key).unwrap().clone();
            h.remove(&key);
            h.insert(key.clone(), num + 1);

            if num + 1 >= 5 {
                println!("{:?}", h_init.get(&key));
                break;
            }
        } else {
            h.insert(key.clone(), 1);
            h_init.insert(key, ans);
        }
        n += 1;
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
