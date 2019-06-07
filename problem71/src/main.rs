/// p / q < 3 / 7
/// = 7p < 3q
/// = 7p <= 3q - 1
/// = p <= (3q - 1) / 7
/// 
/// and
/// 
/// previously best found value = r / s
/// r / s < p / q
/// = r * q < p * s
/// then new best value p / q
fn main() {
    let mut r = 0;
    let mut s = 1;

    for q in 2..1000001 {
        let p = (3 * q - 1) / 7;
        if p * s > r * q {
            s = q;
            r = p;
        }
    }

    println!("{} {}", s, r);
}

