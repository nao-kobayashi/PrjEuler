fn main() {
    let mut valids = Vec::new();
    for i in 1..=11 {
        let n = valid_formula(i);
        valids.push(n);
        println!("{}:{}", i, n);
    }

    let mut ans = 0;
    for n in 1..=10 {
        let mut result = 0;
        for i in 1..=n {
            let mut temp1 = 1;
            let mut temp2 = 1;

            for j in 1..=n {
                if i == j { continue; }

                temp1 *= n + 1 - j;
                temp2 *= i - j;
            }

            result += temp1 * valids[(i - 1) as usize] / temp2;
        }
        ans += result;

        println!("result {}", result);
        println!("fits {}", ans);
    }

    println!("answer:{}", ans);
}


fn valid_formula(n: i128) -> i128 {
    1 - n 
    + n.pow(2) - n.pow(3)
    + n.pow(4) - n.pow(5)
    + n.pow(6) - n.pow(7)
    + n.pow(8) - n.pow(9)
    + n.pow(10)
     //1 − n + n2 − n3 + n4 − n5 + n6 − n7 + n8 − n9 + n10
}