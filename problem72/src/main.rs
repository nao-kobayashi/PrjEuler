fn main() {
    let limit = 1000000;
    let mut nums = (0..limit + 1).map(|n| n).collect::<Vec<u32>>();

    for p1 in 2..limit + 1 {
        if nums[p1 as usize] == p1 {
            let mut p2 = 1;
            while p1 * p2 <= limit {
                let index = (p1 * p2) as usize;
                nums[index] -= nums[index] / p1;
                p2 += 1;
            }
        }
    }

    let mut answer: u128 = 0;
    for i in 2..limit + 1 {
        answer += nums[i as usize] as u128;
    }

    println!("{}", answer);

}

