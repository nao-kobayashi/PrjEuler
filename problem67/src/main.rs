use std::cmp::max;

fn main() {
    let text_contents = std::fs::read_to_string("./p067_triangle.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            let line_vec = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            data.push(line_vec);
        }
    }

    let mut index = data.len() - 1;
    let mut calculated = Vec::new();

    while index > 0 {
        let from_data = if calculated.len() == 0 {
            data[index].clone()
        } else {
            calculated.clone()
        };
        calculated.clear();

        for (i, n) in data[index - 1].iter().enumerate() {
            calculated.push(n + max(from_data[i], from_data[i + 1]));
        }

        index -= 1;
    }

    println!("{:?}", calculated);

}
