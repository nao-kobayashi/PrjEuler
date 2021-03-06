use std::time::Instant;

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

//425185
fn main() {
    let start = Instant::now();

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
            nodes.push(Node::new(r, c, max_row, max_col))
        }
    }

    nodes[0].cost = 0;
    loop {
        //確定対象ノードを選択する。
        //BinaryHeapを使ってもOK
        let mut done_node: i32 = -1;
        for (i, node) in nodes.iter().enumerate() {
            if node.done || node.cost < 0 {
                continue;
            }

            //一番小さいのを対象としたいのでbreakしてはダメ
            if done_node == -1 || node.cost < nodes[done_node as usize].cost {
                done_node = i as i32;
            }
        }
        //確定ノードのみになれば終了
        if done_node == -1 { break; }

        //更新対象を集めておく
        let node_index = done_node as usize;
        nodes[node_index].done = true;
        let mut update_source = Vec::new();
        for t in nodes.get(node_index).unwrap().get_connect() {
            let cost = nodes[node_index].cost + data[t.0][t.1];
            if let Some(index) = nodes.iter().position(|n| n.col == t.1 && n.row == t.0) {
                if nodes[index].cost < 0 || cost < nodes[index].cost {
                    update_source.push((index, cost, nodes[node_index].row, nodes[node_index].col));
                }
            }
        }

        //更新
        for (index, cost, from_row, from_col) in update_source {
            nodes[index].cost = cost;
            nodes[index].from_row = from_row;
            nodes[index].from_col = from_col;
        }
    }

    //nodes.iter().for_each(|node| println!("{:?}", node));
    //スタート地点のコストを足す
    println!("{:?}", nodes[nodes.len() - 1].cost + data[0][0]);
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

#[derive(Debug)]
struct Node {
    pub connect: Vec<(usize, usize)>,
    pub done: bool,
    pub cost: i32,
    pub row: usize,
    pub col: usize,
    pub from_row: usize,
    pub from_col: usize,
}

impl Node {
    pub fn new(row: usize, col: usize, max_row: usize, max_col: usize, ) -> Self {
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

        //start地点への接続はなし
        if let Some(pos) = connect.iter().position(|t| t.0 == 0 && t.1 == 0) {
            connect.remove(pos);
        }

        Self {
            connect,
            row,
            col,
            done: false,
            cost: -1,
            from_row: 0,
            from_col: 0,
        }
    }

    pub fn get_connect(&self) -> &[(usize, usize)] {
        &self.connect
    }
}