fn main() {
    let mut a: u128 = 2;

    for _ in 0..7830457 - 1 {
    // for _ in 0..100000 - 1 {
        a *= 2;

        if a >= 10000000000 {
            let s = a.to_string();
            let l = s.len() - 1;
            let cnv = s[l-9..=l].to_string();
            a = cnv.parse::<u128>().unwrap();
        }
    }

    println!("{}", 28433 * a + 1);
}
