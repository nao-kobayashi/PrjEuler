use std::time::Instant;
use std::collections::HashMap;

fn get_data(except: &Vec<u32>) -> Vec<Vec<u32>> {
    let text_contents = std::fs::read_to_string("./p079_keylog.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            let line_vec = line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .filter(|n| except.iter().all(|e| e != n))
                .map(|n| n)
                .collect::<Vec<u32>>();

            if line_vec.len() > 0 {
                data.push(line_vec);
            }
        }
    }
    data
}

fn main() {
    let start = Instant::now();

    let mut answer = Vec::new();
    loop {
        let data = get_data(&answer);
        if data.len() == 0 { break; }

        let mut hash = HashMap::new();
        for line in data.iter() {
            let n = line[0];
            let mut count = 0;
            for line2 in data.iter() {
                for n2 in 0..line2.len() {
                    if n2 == 0 { continue; }
                    if n == line2[n2] {
                        count += 1;
                    }
                }
            }
            hash.insert(n, count);
        }

        //println!("{:?}", hash);
        //countが一番少ない値
        let mut min = 9999999;
        let mut index = 0;
        for (k, v) in hash.clone() {
            if v < min {
                min = v;
                index = k;
            }
        }
        answer.push(index);
    }

    println!("{:?}", answer);
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
