/*
def part(n, m = n)
    if m == 0
        0
    elsif n <= 0 or m == 1
        1
    elsif n < m
        part(n, n)
    else
        @part ||= {}
        @part[[n, m]] ||= part(n - m, m) + part(n, m - 1)
    end
end

p part(100) 
*/

// p(n, m) nという数字をm以下数字の足し算で表す。
// => p(n, m) - p(n, m - 1) = p(n - m, m)
// p(n, m) = p(n, m - 1) + p(n - m, m)
fn main() {
    let target = 100;
    let mut ways = (0..target + 1).map(|_n| 0).collect::<Vec<u64>>();
    part(target, target, &mut ways);
    println!("{:?}", ways.iter().map(|n| *n).sum::<u64>() - 1);
    println!("{:?}", ways);



    // let target = 10;
    // let mut ways = (0..target + 1).map(|_n| 0).collect::<Vec<u64>>();
    // let mut history = Vec::new();
    // ways[0] = 1;

    // for i in 1..target {
    //     for j in i..target + 1 {
    //         ways[j] += ways[j - i];
    //     }

    //     history.push(ways.clone());
    // }

    // println!("{:?}", ways[ways.len() - 1]);
    // println!("{:?}", history);
}

fn part(n: i32, m: i32, ways: &mut Vec<u64>) -> u64 {
    if m == 0 {
        0
    } else if n <= 0 || m == 1 {
        1
    } else if n < m {
        part(n, n, ways)
    } else {
        ways[n as usize] += part(n - m, m, ways) + part(n, m - 1, ways);
        //println!("n:{} = {}", n, part(n - m, m, ways) + part(n, m - 1, ways));
        0
    }
}

// 1   2   3   4   5   6    7    8    9   10
// 1   1   2   4   6   10   14   21   29  41
/*fn main() {
    let mut coins = (1..101).map(|n| n).collect::<Vec<i32>>();
    coins.sort_by_key(|n| -1 * n);
    ///println!("{:?}", coins);


    for i in 1..11 {
        let mut count = 0;
        problem31(&coins, 0, 0, i, &mut count);
        println!("{} = count:{}", i, count - 1);
    }

}

fn problem31(coins: &[i32], index: usize, sum: i32, target: i32, count: &mut i32) {
    let n = coins[index];
    let prv_val = sum;

    for i in 0..(target / n + 1) {
        let val = (i * n) + prv_val;

        if val == target {
            *count += 1;
        } else {
            if index + 1 < coins.len() {
                problem31(coins, index + 1, val, target, count);
            }
        }
    }
}
*/