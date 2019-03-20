extern crate num_bigint;
use num_bigint:: BigInt;
extern crate num_traits;
use num_traits::pow::Pow;

use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::{ max, min };
use std::thread::spawn;

fn problem31(coins: &[i32], index: usize, sum: i32, count: &mut i32) {
    let n = coins[index];
    let prv_val = sum;

    for i in 0..(200 / n + 1) {
        let val = (i * n) + prv_val;

        if val == 200 {
            *count += 1;
        } else {
            if index + 1 < coins.len() {
                problem31(coins, index + 1, val, count);
            }
        }
    }
}

fn erase_num(hash: &mut HashSet<u32>, expected: i32) -> bool {
    let nums = expected.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    for n in nums {
        if hash.contains(&n) {
            hash.remove(&n);
        } else {
            return false;
        }
    }

    true
}

fn problem32() {
    let mut hash = HashSet::new();
    (1..10).for_each(|n| { hash.insert(n); });

    //多分、2桁まで * 4桁まで の組み合わせしか条件を満たさないはず。
    let mut answers = HashSet::new();
    for i in 2..100 {
        let mut hash_i = hash.clone();
        if erase_num(&mut hash_i, i) {
            for h in 2..10000 {
                let mut hash_h = hash_i.clone();
                if erase_num(&mut hash_h, h) {
                    let z = i * h;
                    if z.to_string().len() == hash_h.len() && erase_num(&mut hash_h, z) {
                        if !answers.contains(&z) {
                            answers.insert(z);
                            println!("{} * {} = {}", i, h, z);
                        }
                    }
                }
            }
        }
    }

    //println!("{:?}", answers);
    println!("{}", answers.iter().map(|n| *n).sum::<i32>());
}

fn erase_num2(i: i32, h: i32, z: i32) -> (bool, i32, i32) {
    let cnv_i = i.to_string().replace(&z.to_string(), "").parse::<i32>().unwrap_or(i);
    if cnv_i != i {
        let cnv_h = h.to_string().replace(&z.to_string(), "").parse::<i32>().unwrap_or(h);
        if cnv_h != h {
            return (true, cnv_i, cnv_h);
        }
    }
    (false, i, h)
}

fn problem33() {
    let mut ans = (1, 1);
    for i in 11..100 {
        if i % 10 == 0 { continue; }
        for h in (i + 1)..100 {
            if h % 10 == 0 { continue; }

            let z = i as f64 / h as f64 ;
            let mut numbers = i.to_string().chars()
                .chain(h.to_string().chars())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            numbers.dedup();

            for n in numbers {
                let (b, cnv_i, cnv_h) = erase_num2(i, h, n as i32);
                if b && cnv_i as f64 / cnv_h as f64  == z {
                    println!("{} {} {}\t{} {} {}", i, h, z, cnv_i, cnv_h, z);
                    ans.0 = ans.0 * cnv_i;
                    ans.1 = ans.1 * cnv_h;
                }
            }
        }
    }
    println!("ans {:?}", ans);
}

//9!7 = 2540160
//9!8 = 2903040
//より2540160までのチェックでOKのはず。
//(8桁まで行くと8桁の最大値(9!8)で階乗してもソースの値の方がでかくなる。）
fn problem34() {
    let mut answer = 0;
    let mut list = (1..10)
        .map(|n| (1..(n + 1)).fold(1, |a, b| a * b))
        .collect::<Vec<i32>>();

    //0の階乗分
    list.insert(0, 1);

    let answer: i32 = (3..2540160 + 1)
        .map(|n| {
            if n.to_string().chars()
                .map(|c| *list.get(c.to_digit(10).unwrap() as usize).unwrap())
                .sum::<i32>()
                .eq(&n) {

                println!("calc: {}", n);
                n
            } else {
                0
            }
        })
        .sum();

    println!("answer: {}", answer);
}

fn is_circular_primes(n: i32, primes: &mut HashSet<i32>) -> bool {
    let mut i = 0_usize;
    //これをループの中に入れると101とか0が先頭に来た時に値が変わってしまう。
    let mut snum = n.to_string();

    while i < n.to_string().len() {
        //巡回
        let first = snum.drain(0..1).max().unwrap();
        snum.push(first);

        if !is_prime(snum.parse::<i32>().unwrap()) {
            return false;
        }
        i += 1;
    }

    //hashから自分を消す。消さないと2重にカウントされる。例（17 <--> 71）
    primes.remove(&n);
    true
}

fn problem35() {
    let primes = (0..1000000)
        .filter_map(|n|
            if is_prime(n) {
                Some(n)
            } else {
                None
            }
        )
        .collect::<HashSet<i32>>();

    let mut i = 0;
    let mut primes2 = primes.clone();
    let ans = primes.iter()
        .filter_map(|n|
            if is_circular_primes(*n, &mut primes2) {
                Some(1)
            } else {
                None
            }
        )
        .sum::<i32>();

    println!("{:?}", ans);
}


fn problem36() {
    let sum = (1..1000000)
        .filter(|n| is_palindrome(&n.to_string()) && is_bin_palindrome(*n))
        .map(|n| n)
        .sum::<i32>();

    println!("problem36 is {}.", sum);
}

fn is_bin_palindrome(n: i32) -> bool {
    let s = format!("{:b}", n); //format_bin(n);
    is_palindrome(&s)
}

fn format_bin(n: i32) -> String {
    let mut s = format!("{:b}", n);
    if s.len() % 4 != 0 {
        let add = 4 - (s.len() % 4);
        for _ in 0..add {
            s = "0".to_string() + &s;
        }
    }
    s
}

fn problem37() {
    let mut num = 11;
    let mut ans = Vec::new();

    while ans.len() < 11 {
        if is_prime(num) {
            //左を削っていく
            if remove_left(num) && remove_right(num) {
                ans.push(num);
            }
        }

        num += 1;
    }

    println!("{}", ans.iter().map(|n| *n).sum::<i32>());
}

fn remove_left(n: i32) -> bool {
    let s = n.to_string();
    let v = s.chars().map(|c| c as u8).collect::<Vec<u8>>();
    let cnv = String::from_utf8_lossy(&v[1..s.len()]);
    let n2 = cnv.parse::<i32>().unwrap();
    if is_prime(n2) {
        if n2.to_string().len() > 1 {
            return remove_left(n2);
        } else {
            return true;
        }
    }
    false
}


fn remove_right(n: i32) -> bool {
    let s = n.to_string();
    let v = s.chars().map(|c| c as u8).collect::<Vec<u8>>();
    let cnv = String::from_utf8_lossy(&v[0..s.len() - 1]);
    let n2 = cnv.parse::<i32>().unwrap();
    if is_prime(n2) {
        if n2.to_string().len() > 1 {
            return remove_right(n2);
        } else {
            return true;
        }
    }
    false
}

fn problem38() {
    let mut base = HashSet::new();
    (1..10).for_each(|n| { base.insert(n); });

    let mut i = 1;
    let mut max = 0;
    while i < 1000000 {
        let mut pandigital = String::new();
        for h in 1..100 {
            let work = (i * h).to_string();
            pandigital.push_str(&work);
            let mut copy_num = base.clone();

            if pandigital.len() == 9 {
                for c in pandigital.chars() {
                    let n = c.to_digit(10).unwrap();
                    if copy_num.contains(&n) {
                        copy_num.remove(&n);
                    } else {
                        break;
                    }
                }

                if copy_num.len() == 0 {
                    let maybe = pandigital.parse::<i64>().unwrap();
                    if max < maybe {
                        max = maybe;
                    }
                }
            } else if pandigital.len() > 9 { break; }
        }

        i += 1;
    }

    println!("{}", max);

}

//三平方の定理 斜辺の2乗が残りの2辺の2乗の和に等しい
fn problem39() {
    let limit = 1000;
    let mut hash = HashSet::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in 3..limit {
        for h in 3..limit {
            let z = ((i * i) + (h * h)) as f64;
            let maybe = z.sqrt() as i32;

            if maybe + i + h <= limit && maybe * maybe == (i * i) + (h * h) {
                let mut v = vec![maybe, i, h];
                v.sort();

                if !hash.contains(&v) {
                    hash.insert(v);

                    let key = maybe + i + h;
                    if map.contains_key(&key) {
                        let val = map.get(&key).unwrap();
                        let new = *val + 1;
                        map.insert(key, new);
                    } else {
                        map.insert(key, 1);
                    }
                }

                // println!("{} {} {} z:{}", maybe, i, h, z);
            }
        }
    }

    let mut max_value = 0;
    let mut max_key = 0;

    map.iter()
        .for_each(|(key, value)| {
            if max_value < *value {
                max_value = *value;
                max_key = *key;
            }
        });

    println!("{:?}", map);
    println!("max_key: {:?}", max_key);
    println!("max_value: {:?}", max_value);
}

fn problem40() {
    let mut i = 1;
    let mut mul_cnt = 0;
    let mut target = 1;
    let mut length = 0;
    let mut ans = 1;

    while mul_cnt < 7 {
        if i.to_string().len() + length >= target {
            for c in i.to_string().chars() {
                length += 1;
                if length == target {
                    println!("char:{}  num:{}  target:{}", c, i, target);
                    ans *= c.to_digit(10).unwrap();
                    target *= 10;
                    mul_cnt += 1;
                }
            }
        } else {
            length += i.to_string().len();
        }

        i += 1;
    }

    println!("ans:{}", ans);

}

//Note: Nine numbers cannot be done (1+2+3+4+5+6+7+8+9=45 => always dividable by 3)
// Note: Eight numbers cannot be done (1+2+3+4+5+6+7+8=36 => always dividable by 3)
fn problem41() {
    let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut i = 1;
    let mut n_max = 0;
    for i in 10..10000000 {
        let num_cnt = i.to_string().len();
        let mut keta = num[0..num_cnt].iter()
            .map(|n| *n)
            .collect::<HashSet<i32>>();

        for c in i.to_string().chars() {
            let n = c.to_digit(10).unwrap() as i32;
            if keta.contains(&n) {
                keta.remove(&n);
            } else {
                break;
            }
        }
        if keta.len() == 0 {
            let maybe = i;
            if is_prime(maybe) {
                //println!("{} is prime.", maybe);
                n_max = max(maybe, n_max);
            }
        }
    }

    println!("problem41 is {}.", n_max);
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

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

fn problem42() {
    let text_contents = std::fs::read_to_string("./words.txt").unwrap();
    let replace_string = text_contents.replace("\"", "").replace("\n", "");
    let mut names = replace_string.split(",").map(|s| s.to_string().to_uppercase()).collect::<Vec<String>>();
    names.sort();

    let triangle_nums = (1..100)
        .map(|n| n * (n + 1) / 2)
        .collect::<HashSet<i32>>();

    let mut nums = Vec::new();
    for word in names.iter() {
        let num = word.chars()
            .map(|c| c as i32 - 64)
            .sum::<i32>();

        nums.push(num);
    }

    let cnt = nums.iter()
        .filter(|n| triangle_nums.contains(&n) )
        .count();
    
    println!("{}", cnt);
}

//これ長時間かかる 注意
fn problem43() {
    let mut sum = 0;
    for i in 1000000000..10000000000_i64 {
        let s = i.to_string();
        if &s[7..10].parse::<i32>().unwrap() % 17 == 0
            && &s[6..9].parse::<i32>().unwrap() % 13 == 0
            && &s[5..8].parse::<i32>().unwrap() % 11 == 0
            && &s[4..7].parse::<i32>().unwrap() % 7 == 0
            && &s[3..6].parse::<i32>().unwrap() % 5 == 0
            && &s[2..5].parse::<i32>().unwrap() % 3 == 0
            && &s[1..4].parse::<i32>().unwrap() % 2 == 0 {

            //println!("target: {:?}", i);
            let mut keta = s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            keta.sort();
            keta.dedup();

            if keta.len() == 10 {
                sum += i;
                println!("found: {:?}", i);
            }
        }
    }
    println!("answer:{:?}", sum);
}

fn problem44() {
    let pentagon = (1..3001).map(|n| (n * (3 * n - 1)) /2 ).collect::<Vec<i32>>();
    let hash = pentagon.iter().map(|n| *n).collect::<HashSet<i32>>();

    let mut min_n: i32 = 999999999;
    let mut index = pentagon.len() - 1;
    while index >= 0 {
        let num1 = pentagon[index];
        let mut index2 = pentagon.len() - 1;
        while index2 >= 0 {
            let num2 = pentagon[index2];
            if hash.contains(&(num2 - num1)) && hash.contains(&(num2 + num1)) {
                min_n = min(min_n, num2 - num1);
            }
            if index2 == 0 { break; }
            index2 -= 1;
        }

        if index == 0 { break; }
        index -= 1;
    }

    println!("{:?}", min_n);
}

fn problem45() {
    let max = 500000_i64;
    let triangle = (1..max).map(|n| (n * (n + 1)) /2 ).collect::<Vec<i64>>();
    let pentagon = (1..max).map(|n| (n * (3 * n - 1)) /2 ).collect::<HashSet<i64>>();
    let hexagonal = (1..max).map(|n| n * (2 * n - 1)).collect::<HashSet<i64>>();

    for t in triangle {
        if pentagon.contains(&t) && hexagonal.contains(&t) {
            println!("{}", t);
        }
    }
}

fn is_twice_square(n: i64) -> bool {
    let square: f64 = (n as f64 / 2.0).sqrt();
    (square as i64) as f64 == square
}

fn problem46() {
    let mut i = 1;
    let mut primes = Vec::new();
    while (primes.len() <= 10000) {
        if is_prime(i) {
            primes.push(i);
        }
        i += 1;
    }

    let mut result = 1;
    let mut found = false;
    while(!found) {
        result += 2;

        let mut j = 0;
        found = true;
        while ( result >= primes[j] ){
            if is_twice_square((result - primes[j]) as i64) {
                found = false;
                break;
            }
            j += 1;
        }
    }

    println!("{}", result);
}

fn problem47() {
    let max = 1000001;
    let mut hash = HashMap::new();
    for i in 1..max {
        let f = factoring(i as f64);
        let v = f.iter().map(|(x, r)| {
            let mut ans = 1;
            for _ in 1..(*r+1) {
                ans *= x;
            }
            ans
        }).collect::<Vec<i64>>();
        hash.insert(i, v);
    }

    for i in 4..max {
        let a = hash.get(&(i - 3)).unwrap();
        let b = hash.get(&(i - 2)).unwrap();
        let c = hash.get(&(i - 1)).unwrap();
        let d = hash.get(&(i)).unwrap();

        let mut v = a.iter()
            .cloned()
            .chain(
                b.iter().cloned()
            ).
            chain(
                c.iter().cloned()
            ).
            chain(
                d.iter().cloned()
            )
            .into_iter()
            .map(|n| n)
            .collect::<Vec<i64>>();

        v.sort();
        v.dedup();
        if v.len() == 16 {
            println!("found:{}", i - 3);
            break;
        }
    }

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

fn problem48() {
    let mut ans = BigInt::from(0);
    for i in 1..1001 {
        let b = BigInt::from(i);
        //println!("{}", b.pow(i as u32));
        ans += b.pow(i as u32);
    }

    println!("{}", ans);
}

fn problem49() {
    let primes = (1000..10000)
        .filter_map(|n| if is_prime(n) { Some(n) } else { None })
        .collect::<Vec<i32>>();

    let hash_primes = primes.iter().map(|n| *n).collect::<HashSet<i32>>();

    for p in primes.iter() {
        for q in primes.iter() {
            if q - p > 0 {
                let diff = q - p;
                let r = q + diff;
                if hash_primes.contains(&r) {
                    let mut p_v = p.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
                    let mut q_v = q.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
                    let mut r_v = r.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
                    p_v.sort();
                    q_v.sort();
                    r_v.sort();

                    let mut flg = true;
                    for (i, n) in p_v.iter().enumerate() {
                        if *n != q_v[i] || *n != r_v[i] {
                            flg = false;
                            break;
                        }
                    }

                    if (flg) {
                        println!("{}{}{}", p, q, r);
                    }
                }
            }
        }
    }

    //println!("{:?}", primes);
}

fn problem50() {
    let primes = (1..1000000)
        .filter_map(|n| 
            if is_prime(n) { 
                Some(n) 
            } else { 
                None 
            }
        )
        .collect::<Vec<i32>>();

    let hash_primes = primes.iter().map(|n| *n).collect::<HashSet<i32>>();

    let max_prime = primes.iter().map(|n| *n).max().unwrap();

    let mut max_count = 0;
    let mut target = 0;
    for n in primes.iter() {
        let mut cnt = 2;
        let mut sum = *n;
        for n2 in primes.iter().filter(|num| num > &n) {
            sum += n2;
            if hash_primes.contains(&sum) {
                if max_count < cnt {
                    target = sum;
                    max_count = cnt;
                    // println!("{} {}", target, max_count);
                }
            }
            cnt += 1;
            if max_prime <= sum { break; }
        }
    }

    println!("{} {}", target, max_count);
}

fn replace(n: i32, rep: &str, to_rep: &str) -> i32 {
    let rep_str = n.to_string().replace(rep, to_rep);
    rep_str.parse::<i32>().unwrap()
}

fn problem51() {
    let mut n = 60000;
    loop {
        if is_prime(n) {
            let v = n.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            //8個置換するはずだから最少値は0 or 1 or 2
            for i in 0..3 {
                //0 or 1 or 2が2つ以上あったら
                let a0 = v.iter().filter(|n| **n == i).count();
                if a0 >= 2 {
                    let mut cnt = 0;
                    for h in 0..10 {
                        let cnv = replace(n, &i.to_string(), &h.to_string());
                        if cnv.to_string().len() == n.to_string().len() && is_prime(cnv) {
                            cnt += 1;
                            if cnt == 8 {
                                println!("found:{} {}", n, i);
                                return;
                            }
                        }
                    }
                }
            }
        }
        n += 1;
    }
}

#[allow(dead_code)]
fn main() {
    let start = Instant::now();

    // let mut coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    // coins.sort_by_key(|n| -1 * n);
    // let mut count = 0;
    // problem31(&coins, 0, 0, &mut count);
    // println!("count:{}", count);

    // problem32();
    // problem33();
    // problem34();
    // problem35();
    // problem36();
    // problem37();
    // problem38();
    // problem38();
    // problem39();
    // problem40();
    // problem41();

    // println!("{}", 'S' as i32 - 64);
    // println!("{}", 'K' as i32 - 64);
    // println!("{}", 'Y' as i32 - 64);

    // println!("{}", 'a' as i32);
    // println!("{}", 's' as i32 - 64);
    // println!("{}", 'k' as i32 - 64);
    // println!("{}", 'y' as i32 - 64);
    //problem42();
    //problem43();
    //problem44();
    //problem45();
    //problem46();

    // println!("14:{:?}", factoring(14.0));
    // println!("15:{:?}", factoring(15.0));
    // println!("644:{:?}", factoring(644.0));
    // println!("645:{:?}", factoring(645.0));
    // println!("646:{:?}", factoring(646.0));
    //problem47();
    //problem48();
    //problem49();
    //problem50();
    problem51();

    println!("{:?}", is_prime(121313));
    println!("{:?}", is_prime(222323));
    println!("{:?}", is_prime(323333));
    println!("{:?}", is_prime(424343));
    println!("{:?}", is_prime(525353));
    println!("{:?}", is_prime(626363));
    println!("{:?}", is_prime(727373));
    println!("{:?}", is_prime(828383));
    println!("{:?}", is_prime(929393));
    //println!("{:?}", is_prime(69191));

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
