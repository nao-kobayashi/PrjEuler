fn main() {
    let max = 10_000_000;
    let mut ans = 0;

    for n in 1..max {
        let mut calc = n;
        while calc != 1 && calc != 89 {
            calc = calc.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|n1| n1 * n1)
                .sum::<u32>();
        }

        if calc == 89 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
