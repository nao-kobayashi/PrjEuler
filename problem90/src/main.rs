extern crate itertools;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let ans = vec![01, 04, 09, 16, 25, 36, 49, 64, 81];

    let mut cnt = 0;
    let arr = create_combination();
    for i1 in 0..arr.len() {
        for i2 in (i1 + 1)..arr.len() {
            let arr1 = &arr[i1];
            let arr2 = &arr[i2];
            
            if check_square_number(arr1, arr2, &mut ans.clone()) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
    let elapsed = start_time.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

fn create_combination() -> Vec<Vec<i32>> {
    (0..10).combinations(6).collect::<Vec<Vec<i32>>>()
}

fn check_square_number(a: &[i32], b: &[i32], ans: &mut Vec<i32>) -> bool {
    let cnv_a = append_if_need(a);
    let cnv_b = append_if_need(b);

    checking(&cnv_a, &cnv_b, ans);
    checking(&cnv_b, &cnv_a, ans);

    ans.len() == 0
}

fn append_if_need(arr: &[i32]) -> Vec<i32> {
    if arr.iter().any(|an| *an == 6 ) && !arr.iter().any(|an| *an == 9 ) {
        let mut cnv_arr = Vec::from(arr);
        cnv_arr.push(9);
        cnv_arr
    } else if !arr.iter().any(|an| *an == 6 ) && arr.iter().any(|an| *an == 9 ) {
        let mut cnv_arr = Vec::from(arr);
        cnv_arr.push(6);
        cnv_arr
    } else {
        Vec::from(arr)
    }
}

fn checking(a: &[i32], b: &[i32], ans: &mut Vec<i32>) {
    for n1 in a {
        for n2 in b {
            let n = (n1.to_string() + &n2.to_string()).parse::<i32>().unwrap();
            if let Some(index) = ans.iter().position(|an| *an == n) {
                ans.remove(index);
                if ans.len() == 0 { return; }
            }
        }
    }
}