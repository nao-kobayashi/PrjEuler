use std::time::Instant;
fn main() {
    let start = Instant::now();
    let target: i128 = 2000000;
    let x_len = ((target as f64).sqrt() + 1.0) as i128;
    let y_len = x_len;

    let mut result = target;
    let mut result_x = 0;
    let mut result_y = 0;

    //(項数 * (2 * 初項 + (項数 - 1 ) * 項差)) / 2
    //(x * (2 + (x - 1) * 1)) / 2
    //(x * (x + 1))/2
    //ex x = 3, => 6
    //y = 2, => 3
    //積の法則 6 * 3 = 18
    // 3 x 2 の長方形は 18 通り
    for y in 1..y_len {
        let y_calc = (y * (y + 1)) / 2;
        for x in 1..x_len {
            let x_calc = (x * (x + 1)) / 2;
            let rect_count = y_calc * x_calc;
            let rect_diff =  i128::abs(rect_count - target);

            if result > rect_diff {
                result = rect_diff;
                result_x = x;
                result_y = y;
            }
        }
    }

    println!("ans:{} x:{} y:{}", result_x * result_y, result_x, result_y);
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
