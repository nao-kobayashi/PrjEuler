fn main() {
    // println!("{}", 3_f64.ln());
    // println!("{}", 2_f64.ln());
    // println!("{}", 1_f64.ln());

    let data = create_data();
    // println!("{:?}", data);

    let mut row_no = 1;
    let mut ans = 0;
    let mut bigger = 0.0;
    for (x, y) in data {
        let w = y * x.ln();
        if bigger < w {
            bigger = w;
            ans = row_no;
        }
        row_no += 1;
    }

    println!("answer:{}", ans);
}

fn create_data() -> Vec<(f64, f64)> {
    let text_contents = std::fs::read_to_string("./p099_base_exp.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            let tokens = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            data.push((
                tokens[0].parse::<f64>().unwrap(),
                tokens[1].parse::<f64>().unwrap(),
                ));
        }
    }
    data
}
