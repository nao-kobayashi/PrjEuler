#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

///= 0 - 直線上に線分の1点 or 2点がある
///< 0 - 直線と線分が交差する
///> 0 - 直線と線分は交差しない
fn is_cross(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> f64 {
    ((p1.x - p2.x) * (p3.y - p1.y) + (p1.y - p2.y) * (p1.x - p3.x)) 
    * ((p1.x - p2.x) * (p4.y - p1.y) + (p1.y - p2.y) * (p1.x - p4.x))
}

fn is_in_triangle(tp1: &Point, tp2: &Point, tp3: &Point, xp: &Point) -> bool {
    //三角形の平均値を求める。
    let avg = Point {
        x: (tp1.x + tp2.x + tp3.x) / 3.0,
        y: (tp1.y + tp2.y + tp3.y) / 3.0,
    };

    //3点の中心と任意の点を結んだときに三角形の線と交差するなら任意の点は三角形の中に無い
    is_cross(tp1, tp2, xp, &avg) > 0.0
    && is_cross(tp2, tp3, xp, &avg) > 0.0
    && is_cross(tp1, tp3, xp, &avg) > 0.0
}

fn main() {
    // let a = Point {x: -340.0, y: 495.0};
    // let b = Point {x: -153.0, y: -910.0};
    // let c = Point {x: 835.0, y: -947.0};

    // let x = Point {x: -175.0, y: 41.0};
    // let y = Point {x: -421.0, y: -714.0};
    // let z = Point {x: 574.0, y: -645.0};
    
    // let org = Point {x: 0.0, y: 0.0};

    // println!("三角形ABC include origin {}", is_in_triangle(&a, &b, &c, &org));
    // println!("三角形XYZ include origin {}", is_in_triangle(&x, &y, &z, &org));

    let mut ans = 0;
    let org = Point {x: 0.0, y: 0.0};
    for (tp1, tp2, tp3) in get_data() {
        if is_in_triangle(&tp1, &tp2, &tp3, &org) {
            ans += 1;
        }
    }

    println!("include origin count:{}", ans);
}

fn get_data() -> Vec<(Point, Point, Point)> {
    let text_contents = std::fs::read_to_string("./p102_triangles.txt").unwrap();
    let all_lines = text_contents.split("\n");
    let mut data = Vec::new();
    for line in all_lines {
        if line != "" {
            let tokens = line.split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            if tokens.len() == 6 {
                data.push((
                    Point {x: tokens[0].parse::<f64>().unwrap(), y: tokens[1].parse::<f64>().unwrap()},
                    Point {x: tokens[2].parse::<f64>().unwrap(), y: tokens[3].parse::<f64>().unwrap()},
                    Point {x: tokens[4].parse::<f64>().unwrap(), y: tokens[5].parse::<f64>().unwrap()},
                ));
            }
        }
    }
    data
}