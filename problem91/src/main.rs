fn main() {
    println!("ans {}", calc(50));
}

fn calc(n: i32) -> i32 {
    let mut ans = 0;

    for x1 in 0..=n {
        for y1 in 0..=n {
            if x1 == 0 && y1 == 0 { continue; }

            for x2 in 0..=n {
                for y2 in 0..=n {
                    if x2 == 0 && y2 == 0 { continue; }
                    if x1 == x2 && y1 == y2 { continue; }

                    let xy1 = x1 * x1 + y1 * y1;
                    let xy2 = x2 * x2 + y2 * y2;
                    let zz = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);

                    if xy1 + xy2 == zz || xy1 + zz == xy2 || xy2 + zz == xy1 {
                        ans += 1;
                    }
                }
            }
        }
    }

    //same combination created
    //x1, y1 <==> x2, y2
    ans / 2
}