fn problem1() {
    println!("{}",
        (0..1000)
            .filter(|i| i % 3 == 0 || i % 5 == 0)
            .sum::<i32>()
    );
}

fn problem2(a: i32, b: i32, mut sum: i32) {
    let c = a + b;

    if b % 2 == 0 {
        sum += b;
    }

    if c >= 4000000 {
        println!("answer:{}", sum);
    }else {
        problem2(b, c, sum);
    }
}

fn problem3(n: f64) {
    let s = n.sqrt() as usize;
    let mut y = n as i64;

    for i in 2..s + 1 {
        while y % (i as i64) == 0 {
            println!("{}", i);
            y = y / i as i64;
        }
    }
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

fn problem4() {
//    let mut result = Vec::new();
//    for x in 100..1000 {
//        for y in 100..1000 {
//            let ans = x * y;
//            if is_palindrome(&ans.to_string()) {
//                result.push(ans);
//            }
//        }
//    }
//
//    println!("{}", result.iter().map(|n| *n).max().unwrap());

    println!("{}",
             (100..1000)
                .map(|x|
                    (100..1000)
                        .filter_map(|y|{
                            let n = x * y;
                            if is_palindrome(&n.to_string()) {
                                Some(n)
                            } else {
                                None
                            }
                        })
                        .max()
                        .unwrap_or(-1)
                )
                .max()
                .unwrap()
    );
}

use std::cmp::{ max, min };
fn gcd(a: i64, b: i64) -> i64 {
    let max = max(a, b);
    let min = min(a, b);

    if max % min == 0 {
        min
    } else {
        gcd(min, max % min)
    }
}

fn problem5(a: i64, b: i64, max: i64) {
    let c = a * b / gcd(a, b);

    if b == max {
        println!("{}", c);
        return;
    }

    problem5(c, b + 1, max);
}

fn problem6(s: i32, e: i32) {
    let a: i32 = (s..e + 1)
        .map(|n| n * n)
        .sum();

    let b: i32 = (s..e + 1)
        .map(|n| n)
        .sum();

    println!("{}", b * b - a);
}

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

fn problem7(pos: i32) {
    let mut num = 0;
    let mut cnt = 0;
    while cnt != pos {
        num += 1;
        if is_prime(num) {
            cnt += 1;
        }
    }
    println!("{}", num);
}

fn problem8(mul_num: usize) {
    let data = "73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";

    //改行コードが来てしまう。
    let num_arr = data
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u64)
        .collect::<Vec<u64>>();

    let mut cur_ans = 0;
    let e = num_arr.len() - mul_num - 1;

    for i in 0..e {
        let mut calc = (0..mul_num).fold(1, |mul, h| mul * num_arr[h + i]);
        cur_ans = max(cur_ans, calc);
    }

    println!("{}", cur_ans);
}

fn problem9(ans: i32) {
    for i in 1..ans {
        for h in i + 1..ans {
            for z in h + 1..ans {
                let p1 = (i as f64).powi(2);
                let p2 = (h as f64).powi(2);
                let p3 = (z as f64).powi(2);

                if p1 + p2 == p3 && i + h + z == ans {
                    println!("{} {} {} {:?} {:?} {:?}", i, h, z, p1, p2, p3);
                    println!("ans {}*{}*{}={}", i, h, z, i * h* z);
                    return;
                }
            }
        }
    }
}

fn problem10(max: i32) {
    let ans: i64 = (2..max + 1)
        .filter(|n| is_prime(*n))
        .map(|n| n as i64)
        .sum();

    println!("{}", ans);
}

fn main() {
    //problem1();
    //problem2(1, 2, 0);
    //problem3(600851475143.0);
//    println!("{}", is_palindrome("abcd"));
//    println!("{}", is_palindrome("abba"));
    //problem4();
    //problem5(1, 2, 11);
    //problem6(1, 100);

//    println!("{}", is_prime(19));
    //problem7(10001);
    //problem8(13);
    //problem9(1000);
    problem10(2000000);
}
