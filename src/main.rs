use std::time::Instant;

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

fn problem11() {
    let data = [[08,02,22,97,38,15,00,40,00,75,04,05,07,78,52,12,50,77,91,08],
        [49,49,99,40,17,81,18,57,60,87,17,40,98,43,69,48,04,56,62,00],
        [81,49,31,73,55,79,14,29,93,71,40,67,53,88,30,03,49,13,36,65],
        [52,70,95,23,04,60,11,42,69,24,68,56,01,32,56,71,37,02,36,91],
        [22,31,16,71,51,67,63,89,41,92,36,54,22,40,40,28,66,33,13,80],
        [24,47,32,60,99,03,45,02,44,75,33,53,78,36,84,20,35,17,12,50],
        [32,98,81,28,64,23,67,10,26,38,40,67,59,54,70,66,18,38,64,70],
        [67,26,20,68,02,62,12,20,95,63,94,39,63,08,40,91,66,49,94,21],
        [24,55,58,05,66,73,99,26,97,17,78,78,96,83,14,88,34,89,63,72],
        [21,36,23,09,75,00,76,44,20,45,35,14,00,61,33,97,34,31,33,95],
        [78,17,53,28,22,75,31,67,15,94,03,80,04,62,16,14,09,53,56,92],
        [16,39,05,42,96,35,31,47,55,58,88,24,00,17,54,24,36,29,85,57],
        [86,56,00,48,35,71,89,07,05,44,44,37,44,60,21,58,51,54,17,58],
        [19,80,81,68,05,94,47,69,28,73,92,13,86,52,17,77,04,89,55,40],
        [04,52,08,83,97,35,99,16,07,97,57,32,16,26,26,79,33,27,98,66],
        [88,36,68,87,57,62,20,72,03,46,33,67,46,55,12,32,63,93,53,69],
        [04,42,16,73,38,25,39,11,24,94,72,18,08,46,29,32,40,62,76,36],
        [20,69,36,41,72,30,23,88,34,62,99,69,82,67,59,85,74,04,36,16],
        [20,73,35,29,78,31,90,01,74,31,49,71,48,86,81,16,23,57,05,54],
        [01,70,54,71,83,51,54,69,16,92,33,48,61,43,52,01,89,19,67,48]
    ];

    let mut values = Vec::new();
    for x in 0..20 {
        for y in 0..20 {
            if x < 17 as usize {
                let h1 = data[y][x];
                let h2 = data[y][x + 1];
                let h3 = data[y][x + 2];
                let h4 = data[y][x + 3];
                values.push(h1 * h2 * h3 * h4);
            }

            if x < 17 as usize && y < 17 as usize {
                let c_r1 = data[y][x];
                let c_r2 = data[y + 1][x + 1];
                let c_r3 = data[y + 2][x + 2];
                let c_r4 = data[y + 3][x + 3];
                values.push(c_r1 * c_r2 * c_r3 * c_r4);
            }

            if y < 17 as usize {
                let v1 = data[x][y];
                let v2 = data[x][y + 1];
                let v3 = data[x][y + 2];
                let v4 = data[x][y + 3];
                values.push(v1 * v2 * v3 * v4);
            }

            if x > 2 as usize && y < 17 as usize {
                let c_l1 = data[y][x];
                let c_l2 = data[y + 1][x - 1];
                let c_l3 = data[y + 2][x - 2];
                let c_l4 = data[y + 3][x - 3];
                values.push(c_l1 * c_l2 * c_l3 * c_l4);
            }
        }
    }

    println!("{}", values.iter().map(|n| n).max().unwrap());
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

//fn divisors(n: i32) -> Vec<i32> {
//    (1..n + 1)
//        .filter_map(|d|{
//            if n % d == 0 {
//                Some(d)
//            } else {
//                None
//            }
//        })
//        .collect::<Vec<i32>>()
//}

fn problem12(target: i32) {
    let ans = (1..std::i32::MAX)
        .find_map(|n| {
            let tr: i32 = (1..n + 1).map(|t| t).sum();

            //素因数分解した結果の指数部それぞれに1足して全部掛け合わせると約数の数になる
            let ans = factoring(tr as f64)
                .iter()
                .fold(1, |calc, (num, r)| calc * (r + 1));

            if ans > target {
                Some(tr)
            } else {
                None
            }
        });

    println!("{:?}", ans.unwrap_or(-1));
}

extern crate fnv;
use fnv::FnvHashMap;
fn problem14() {
    let mut ans = 0;
    let mut max_num = 0;
    let mut cache = FnvHashMap::default();

    for i in 2..1000000 {
        let mut ret: i64 = i;
        let mut count: i64 = 1;

        while ret != 1 {
            ret = if ret % 2 == 0 {
                ret / 2
            } else {
                ret * 3 + 1
            };

            if cache.contains_key(&ret) {
//                println!("cache hit loop in {} at {}", i, ret);
                count += cache[&ret];
                break;
            }
            count += 1;
        }

        max_num = max(max_num, count);
        if count == max_num {
            ans = i;
        }

        //println!("{} {}", i, count);
        cache.insert(i, count);
    }

    println!("answer:{}", ans);
}

fn main() {
    let start = Instant::now();
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
    //problem10(2000000);
    //problem11();
    //problem12(500);
    problem14();


    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

}
