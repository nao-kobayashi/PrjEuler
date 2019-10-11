use std::time::Instant;

struct Sudoku {
    pub values: Vec<Vec<u8>>,
}

impl Sudoku {
    pub fn new(data: Vec<String>) -> Self {
        let mut values = Vec::new();

        for line in data {
            values.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>(),
            );
        }

        Self { values }
    }

    fn target_range(&self, pos: usize) -> (usize, usize) {
        if pos < 3 {
            (0, 2)
        } else if pos >= 3 && pos < 6 {
            (3, 5)
        } else {
            (6, 8)
        }
    }

    pub fn is_valid(&self, row: usize, col: usize, val: u8) -> bool {
        let row_values = &self.values[row];

        //同じ行に同じ値があればNG
        if row_values.iter().any(|n| n == &val) {
            return false;
        }

        //同じ列に同じ値があればNG
        if self.values.iter().map(|line| line).any(|v| v[col] == val) {
            return false;
        }

        // 3 * 3 のマス内に同じ値があったらNG
        let row_range = self.target_range(row);
        let col_range = self.target_range(col);

        if self.values[row_range.0..=row_range.1]
            .iter()
            .flat_map(|line| line[col_range.0..=col_range.1].iter())
            .any(|n| *n == val)
        {
            return false;
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if self.values[r][c] == 0 {
                    for i in 1..10 {
                        if self.is_valid(r, c, i) {
                            self.values[r][c] = i;

                            if self.solve() {
                                return true;
                            }
                        }
                    }
                    self.values[r][c] = 0;
                    return false;
                }
            }
        }
        true
    }

    pub fn answer(&self) -> u32 {
        let num = format!(
            "{}{}{}",
            self.values[0][0], self.values[0][1], self.values[0][2]
        );
        num.parse::<u32>().unwrap()
    }
}

fn create_data() -> Vec<String> {
    // vec![
    //     "003020600".to_string(),
    //     "900305001".to_string(),
    //     "001806400".to_string(),
    //     "008102900".to_string(),
    //     "700000008".to_string(),
    //     "006708200".to_string(),
    //     "002609500".to_string(),
    //     "800203009".to_string(),
    //     "005010300".to_string(),
    // ]

    let text_contents = std::fs::read_to_string("./sudoku.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            data.push(line.replace("\r", "").to_string());
        }
    }
    data
}

fn main() {
    let start = Instant::now();
    let data = create_data();

    //10行ずつ
    let mut index = 0;
    let mut answer = 0;
    while index < data.len() {
        let read_lines = data
            .iter()
            .skip(index + 1)
            .take(9)
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        // println!("{:?}", read_lines);

        let mut sudoku = Sudoku::new(read_lines);
        sudoku.solve();
        // println!("{:?}", sudoku.values);
        
        answer += sudoku.answer();
        index += 10;
        println!("complete {}  answer:{}", index / 10, answer);
    }

    let elapsed = start.elapsed();
    println!(
        "{} ms",
        (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
    );
}
