use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let mut square_numbers: HashMap<usize, Vec<i32>> = HashMap::new();

    let data = read_file();
    let anagrams = get_anagrams(data);

    for (w1, w2) in &anagrams {
        let len = w1.len();
        if square_numbers.contains_key(&len) {
            solve(w1, w2, &square_numbers.get(&len).unwrap());
        } else {
            let nums = create_square_number(len as i32);
            solve(w1, w2, &nums);
            square_numbers.insert(len, nums);
        }
    }
    
    let elapsed = start.elapsed();
    println!(
        "{} ms",
        (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
    );
}

fn solve(w1: &str, w2: &str, square_nums: &[i32]) {
    let mut hash = HashMap::new();
    let mut hash2 = HashMap::new();
    let w1_arr = w1.chars().map(|c| c).collect::<Vec<char>>();
    let w2_arr = w2.chars().map(|c| c).collect::<Vec<char>>();

    for i in 0..square_nums.len() {
        let x = square_nums[i];
        let x_str = x.to_string().chars().map(|c| c).collect::<Vec<char>>();
        let mut check_x = x_str.clone();
        check_x.sort();

        for h in i + 1..square_nums.len() {
            let y = square_nums[h];
            if x == y { continue;}
            let y_str = y.to_string().chars().map(|c| c).collect::<Vec<char>>();

            let mut is_found = true;
            hash.clear(); 
            hash2.clear();

            for (i, c1) in w1_arr.iter().enumerate() {
                if let Some(index) = w2_arr.iter().position(|c| c == c1) {
                    if hash.contains_key(&c1) {
                        if x_str[i] != hash[&c1] || y_str[index] != hash[&c1] {
                            is_found = false;
                            break;
                        }
                    } else {
                        if x_str[i] != y_str[index] {
                            is_found = false;
                            break;
                        }

                        if hash2.contains_key(&x_str[i]) {
                            if hash2[&x_str[i]] != c1 || hash2[&x_str[i]] != &w2_arr[index] {
                                is_found = false;
                                break;
                            }
                        }

                        hash.insert(c1, x_str[i]);
                        hash2.insert(x_str[i], c1);
                    }
                } else {
                    is_found = false;
                    break;
                }
            }

            if is_found {
                //数字自体がアナグラムのはず
                let mut check_y = y_str.clone();
                check_y.sort();
                if check_x != check_y { continue; }

                // println!("{:?}", hash);
                println!("found {} {} {} {}", &w1, &w2, x, y);
                return;
            }
        }
    }
}

fn create_square_number(len: i32) -> Vec<i32> {
    let mut nums = Vec::new();
    let mut start = "1".to_string();
    for _ in 0..len - 1 {
        start += "0";
    }

    let mut start_num = start.parse::<i32>().unwrap();
    let end_enum = (start + "0").parse::<i32>().unwrap();
    while start_num < end_enum {
        let f = start_num as f64;
        let sqrt = f.sqrt();
        if (sqrt as i32 * sqrt as i32) as f64 == f {
            nums.push(start_num);
        }

        start_num += 1;
    }

    nums
}

fn get_anagrams(data: Vec<String>) -> Vec<(String, String)> {
    let mut anagrams = Vec::new();
    for i in 0..data.len() {
        let s1 = data[i].to_string();
        for j in i + 1..data.len() {
            let s2 = data[j].to_string();
            if s1.len() != s2.len() || s1 == s2.chars().rev().collect::<String>() {
                continue;
            }

            let mut arr1 = s1.chars().map(|c| c).collect::<Vec<char>>();
            let mut arr2 = s2.chars().map(|c| c).collect::<Vec<char>>();
            arr1.sort();
            arr2.sort();

            if arr1 == arr2 {
                anagrams.push((s1.to_string(), s2));
            }
        }
    }
    anagrams
}

fn read_file() -> Vec<String> {
    let text_contents = std::fs::read_to_string("./p098_words.txt").unwrap();
    let all_lines = text_contents.split(",");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            data.push(line.replace("\"", "").to_string());
        }
    }
    data
}