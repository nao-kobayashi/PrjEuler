fn get_data() -> Vec<String> {
    let text_contents = std::fs::read_to_string("./p089_roman.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            data.push(line.to_string());
        }
    }

    println!("data len = {}", data.len());
    data
}

fn main() {
    let cnv_val  = vec![
        ("DCCCC","CM"),
        ("LXXXX","XC"),
        ("VIIII","IX"),
        ("CCCC","CD"),
        ("XXXX","XL"),
        ("IIII","IV"),
    ];
    let source = get_data();

    let mut old_sum = 0;
    let mut new_sum = 0;
    for line in source {
        let mut build_string = line.to_string();
        let old_len = line.len();

        for key in 0..cnv_val.len() {
            build_string = build_string.replace(cnv_val[key].0, cnv_val[key].1);
        }

        let new_len = build_string.len();
        // println!("old {} new {}", old_len, new_len);
        old_sum += old_len;
        new_sum += new_len;
    }

    println!("old_sum - new_sum = {}", old_sum - new_sum);
}
