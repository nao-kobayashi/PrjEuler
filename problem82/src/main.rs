use std::cmp::min;

//p082_matrix.txt
fn get_data() -> Vec<Vec<i32>> {
    let text_contents = std::fs::read_to_string("./p082_matrix.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            let line_vec = line.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

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

    let mut ans = data.clone();
    let max_col = data[0].len() - 1;
    let max_row = data.len() - 1;

    let mut cur_col = max_col - 1;
    while cur_col as i32 >= 0 {
        let mut cur_row = max_row;
        while cur_row as i32 >= 0 {
            let own = ans[cur_row][cur_col];

            let right = calc_right(cur_col + 1, cur_row, &ans);
            
            let top = if cur_row > 0 {
                calc_top(cur_col, cur_row - 1, &ans)
            } else {
                std::i32::MAX
            };
            
            let bottom = if cur_row != max_row {
                calc_bottom(cur_col, cur_row + 1, &ans)
            } else {
                std::i32::MAX
            };

            // println!("col {} row {}  top:{} right:{} bottom:{}", cur_col, cur_row, top, right, bottom);
            ans[cur_row][cur_col] = own + min(right, min(top, bottom));

            if cur_row == 0 { break; }
            cur_row -= 1;
        }

        if cur_col == 0 { break; }
        cur_col -= 1;
    }
    
    println!("ans:{:?}", ans.iter().map(|it| it[0]).min());
}

fn calc_right(col: usize, row: usize, data: &Vec<Vec<i32>>) -> i32 {
    data[row][col]
}

fn calc_top(col: usize, row: usize, data: &Vec<Vec<i32>>) -> i32 {
    let a = data[row][col] + data[row][col + 1];
    let b = if row != 0 {
        calc_top(col, row - 1, data) + data[row][col]
    } else {
        std::i32::MAX
    };

    min(a, b)
}

fn calc_bottom(col: usize, row: usize, data: &Vec<Vec<i32>>) -> i32 {
    data[row][col]
}
