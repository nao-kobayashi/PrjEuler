fn is_valid_triangle(one_side: u128, two_sides: u128) -> bool {
    let check = 4 * two_sides * two_sides - one_side * one_side;
    let root = (check as f64).sqrt() as u128;
    check == root * root
}

fn main() {
    let loop_max = 1_000_000_000;
    // let loop_max = 100;
    let mut result = 0;
    let mut perimeter = 16;
    while perimeter <= loop_max + 3 {
        let two_sides = perimeter / 3;
        let one_side = two_sides - 1;
        if is_valid_triangle(one_side, two_sides) {
            println!("{} {} {}", two_sides, two_sides, one_side);
            result += 2 * two_sides + one_side;
        }

        let one_side = two_sides + 1;
        if is_valid_triangle(one_side, two_sides) {
            println!("{} {} {}", two_sides, two_sides, one_side);
            result += 2 * two_sides + one_side;
        }

        perimeter += 3;
    }

    println!("answer:{}", result);
}
