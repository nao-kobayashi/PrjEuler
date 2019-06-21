use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut pentagon = (-100000..100000).map(|n| (n * (3 * n - 1)) /2 ).filter(|n| *n != 0).collect::<Vec<i64>>();
    pentagon.sort_unstable();

    let mut p: Vec<i128> = Vec::new();
    p.push(1);

    let mut n = 1;
    loop {
        let mut i = 0;
        let mut cnt = 1;    //index must start 1, because penta initialized index 0.
        let mut penta = 1;  //initialized index 0 value.
        p.push(0);

        while penta <= n {
            let sign = if i < 2 { 1 } else { -1 };
            if i == 3 { i = 0; } else { i += 1; }

            // println!("n:{} p:{} sign:{} aaa:{} ", n, p[n as usize], sign, p[(n - penta) as usize]);
            p[n as usize] += sign * p[(n - penta) as usize];
            p[n as usize] %= 1000000;
            penta = pentagon[cnt];
            cnt += 1;
        }
        if p[n as usize] == 0 {break;}
        // if n > 5 {break};
        n += 1;
    }

    println!("{:?}", n);
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
