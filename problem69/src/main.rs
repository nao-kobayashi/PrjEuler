fn main() {
    let mut begger_n = 0;
    let mut bigger: f64 = 0.0;
    for n in 2..1_000_000 {
        let f = n as f64;
        let factors = factoring(f);
        //println!("{:?}", factoring(f));
        let mut ans = f;
        for (d, _) in factors {
            let d = d as f64;
            ans *= 1.0 - (1.0 / d);
        }

        
        if bigger < f / ans {
            begger_n = n;
            bigger = f / ans;
        }

        // println!("{} = {}", f, ans);
    }
    println!("answer:{}({})", begger_n, bigger);
}

//素因数分解
fn factoring(n: f64) -> Vec<(i64, i32)> {
    if n == 1.0 {
        return vec![(1, 1)];
    }

    let s = n.sqrt() as usize;
    let mut y = n as i64;
    let mut r = 0;

    let mut result = Vec::new();

    for x in 2..s + 1 {
        if y % (x as i64) == 0 {
            r = 0;
            while y % (x as i64) == 0 {
                r += 1;
                y = y / x as i64;
            }
            result.push((x as i64, r));
        }
    }

    if y as usize > s  {
        result.push((y as i64, 1));
    }

    result
}
