const LIMIT: i32 = 6;

fn main() {
    let limit = LIMIT;
    let mut min_k = vec![99999999; (LIMIT+1) as usize];

    let mut n = 4;
    let mut sum = 0;
    let mut todo = limit - 1;

    while todo > 0 {
        let found = get_min_k(n, n, n, 1, 2, &mut min_k);
        if found > 0 {
            todo -= found;
            sum += n;
        }

        n += 1;
    }

    println!("sum is {}", sum);
}

fn valid(n: i32, k: i32, min_k: &mut Vec<i32>) -> bool {
    if k >= min_k.len() as i32 {
        return false;
    }

    if min_k[k as usize] > n {
        // println!("n={} k={}", n, k);
        min_k[k as usize] = n;
        return true;
    }

    false
}

// return number of minimum k found (a number may be responsible for multiple minimum k, e.g. 8 => k=4 and k=5)
// n: the initial number
// product:   n divided by removed numbers
// sum:       n minus      removed numbers
// depth:     count        removed numbers
// minFactor: skip checking factors smalled than this
fn get_min_k(n: i32, product: i32, sum: i32, depth: i32, min_factor: i32, min_k: &mut Vec<i32>) -> i32 {
    if product == 1 {
        if valid(n, depth + sum - 1, min_k) {
            // println!("n={} depth={} sum={} product={} k={}", n, depth, sum, product, depth + sum - product);
            return 1;
        } else {
            return 0;
        }
    }

    let mut result = 0;
    if depth > 1 {
        if product == sum {
            if valid(n, depth, min_k) {
                // println!("n={} depth={} sum={} product={} k={}", n, depth, sum, product, depth + sum - product);
                return 1;
            } else {
                return 0;
            }
        }

        if valid(n, depth + sum - product, min_k) {
            // println!("n={} depth={} sum={} product={} k={}", n, depth, sum, product, depth + sum - product);
            result += 1;
        }
    }

    let mut i = min_factor;
    while i * i <= product {
        if product % i == 0 {
            result += get_min_k(n, product / i, sum - i, depth + 1, i, min_k);
        }
        i += 1;
    }

    result
}
