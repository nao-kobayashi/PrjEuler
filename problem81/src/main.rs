use std::cmp::min;

fn get_data() -> Vec<Vec<u32>> {
    let text_contents = std::fs::read_to_string("./p081_matrix.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            let line_vec = line.split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if line_vec.len() > 0 {
                data.push(line_vec);
            }
        }
    }
    data
}

fn main() {
    // let data = vec![
    //     vec![131,673,234,103,18],
    //     vec![201,96,342,965,150],
    //     vec![630,803,746,422,111],
    //     vec![537,699,497,121,956],
    //     vec![805,732,524,37,331],
    // ];
    let data = get_data();

    let max_col = (data[0].len() - 1) as i32;
    let max_row = (data.len() - 1) as i32;
    let mut ans = data.clone();

    let mut col: i32 = max_col as i32;
    while col >= 0 {
        let mut row: i32 = max_row as i32;
        while row >= 0 {
            if col != max_col || row != max_row {
                let right = if col < max_col {
                    ans[row as usize][(col + 1) as usize]
                } else {
                    std::u32::MAX
                };

                let bottom = if row < max_row {
                    ans[(row + 1) as usize][col as usize]
                } else {
                    std::u32::MAX
                };

                let lower = min(right, bottom);
                
                //println!("row {} col {} right {} bottom {}  own {}", row, col, right, bottom, ans[row as usize][col as usize]);
                ans[row as usize][col as usize] = lower + ans[row as usize][col as usize];
            }
            row -= 1;
        }

        col -= 1;
    }

    println!("{:?}", ans[0][0]);
}
