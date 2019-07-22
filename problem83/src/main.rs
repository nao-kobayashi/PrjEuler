fn get_data() -> Vec<Vec<i32>> {
    let text_contents = std::fs::read_to_string("./p083_matrix.txt").unwrap();
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

    let mut nodes = Vec::new();
    let max_col = data[0].len() - 1;
    let max_row = data.len() - 1;

    for c in 0..max_col + 1 {
        for r in 0..max_row + 1 {
            nodes.push(Node::new(c, r, max_col, max_row))
        }
    }

    nodes[0].cost = 0;
    loop {
        let mut done_node: i32 = -1;
        for (i, node) in nodes.iter().enumerate() {
            if node.done || node.cost < 0 {
                continue;
            }

            if done_node == -1 || node.cost < nodes[done_node as usize].cost {
                done_node = i as i32;
            }
        }
        if done_node == -1 { break; }

        nodes[done_node as usize].done = true;
        for t in nodes.get(done_node as usize).unwrap().connect.clone() {
            let cost = nodes[done_node as usize].cost + data[t.0][t.1];
            if let Some(index) = nodes.iter().position(|n| n.col == t.1 && n.row == t.0) {
                if nodes[index].cost < 0 || cost < nodes[index].cost {
                    nodes[index].cost = cost;
                }
            }
        }
    }

    // nodes.iter().for_each(|node| println!("{:?}", node));
    println!("{:?}", nodes[nodes.len() - 1].cost + data[0][0]);
}

#[derive(Debug)]
struct Node {
    pub connect: Vec<(usize, usize)>,
    pub done: bool,
    pub cost: i32,
    pub col: usize,
    pub row: usize,
}

impl Node {
    pub fn new(col: usize, row: usize, max_col: usize, max_row: usize) -> Self {
        let mut connect = Vec::new();
        if col != max_col {
            connect.push((row, col + 1));
        }

        if col != 0 {
            connect.push((row, col - 1));
        }

        if row != max_row  {
            connect.push((row + 1, col));
        }

        if row != 0 {
            connect.push((row - 1, col));
        }

        if let Some(pos) = connect.iter().position(|t| t.0 == 0 && t.1 == 0) {
            connect.remove(pos);
        }

        Self {
            connect,
            col,
            row,
            done: false,
            cost: -1,
        }
    }
}


