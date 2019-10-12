//(15/21) * (14/20) = 1/2
//(b / a) * ((b - 1)/(a - 1)) = 1/2
//2*(b / a) * ((b - 1)/(a - 1)) = 1
//2 * (b^2 - b) = (a^2 - a)
//2b^2 - 2b - a^2 + a = 0
//https://www.alpertron.com.ar/QUAD.HTM
//bn+1 = 3 ⁢bn + 2 ⁢an - 2 
//an+1 = 4 ⁢bn + 3 ⁢an - 3

fn main() {
    let mut b: u128 = 15;
    let mut a: u128 = 21;
    let target: u128 = 1000000000000;
    while a < target {
        let work_b = 3 * b + 2 * a - 2;
        let work_a = 4 * b + 3 * a - 3;

        b = work_b;
        a = work_a;
    }

    println!("answer {}", b);
}
