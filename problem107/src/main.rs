use std::mem::swap;
use std::time::Instant;
use std::collections::{ HashMap, HashSet };

fn main() {
    let start = Instant::now();
    let node_size = 40;
    let data = create_data();
    let all_num = data.iter().filter_map(|d| d.weight).sum::<i32>();
    let mut map: HashMap<(usize, usize), i32> = HashMap::new();

    for r in 0..node_size {
        for elm in data.iter().filter(|d| d.row == r && d.weight.is_some()) {
            let mut row = elm.row;
            let mut col = elm.col;

            //eliminate duplication
            if row > col {
                swap(&mut row, &mut col);
            }

            if !map.contains_key(&(row, col)) {
                map.insert((row, col), elm.weight.unwrap());
            }
        }
    }

    let mut ans = 0;
    let mut done: HashSet<usize> = HashSet::new();
    done.insert(0);

    while done.len() < node_size {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut min_weight = 99999999;

        for ((x, y), v) in map.iter() {
            // both nodes are already optimized or both aren't
            let has_from = done.iter().filter(|n| *n == x).count() != 0;
            let has_to = done.iter().filter(|n| *n == y).count() != 0;
            if has_from == has_to {
                continue;
            }

            if min_weight > *v {
                min_x = *x;
                min_y = *y;
                min_weight = *v;
            }
        }

        if min_weight != 99999999 {
            done.insert(min_x);
            done.insert(min_y);
            ans += min_weight;
        }
    }

    println!("{:?}", ans);
    println!("{:?}", all_num / 2 - ans);
    let elapsed = start.elapsed();
    println!(
        "{} ms",
        (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
    );
}

fn create_data() -> Vec<Data> {
    // let text_contents = std::fs::read_to_string("./test.txt").unwrap();
    let text_contents = std::fs::read_to_string("./p107_network.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for (i, line) in all_lines.enumerate() {
        if line != "" {
            let tokens = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            for (h, s) in tokens.iter().enumerate() {
                let n = s.parse::<i32>().ok();
                data.push(
                    Data {
                        row: i,
                        col: h,
                        weight: n,
                    }
                );
            }
        }
    }
    data
}

#[derive(Debug, Clone)]
struct Data {
    pub row: usize,
    pub col: usize,
    pub weight: Option<i32>,
}