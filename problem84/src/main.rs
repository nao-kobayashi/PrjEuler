extern crate rand;
use rand::Rng;
use rand::prelude::*;

const LOOP: i32 = 5000000;
//go 
const GO: usize = 0;
//jail
const JAIL: usize = 10;
//go to jail
const G2JAIL: usize = 30;

fn dice(rng: &mut ThreadRng) -> (bool, i32) {
    // let one = rng.gen_range(1, 7);
    // let two = rng.gen_range(1, 7);
    let one = rng.gen_range(1, 5);
    let two = rng.gen_range(1, 5);
    (one == two, one + two)
}

fn cc_card(cc_count: i32) -> i32 {
    let ret = if cc_count == 0 {
        GO as i32
    } else if cc_count == 1 {
        JAIL as i32
    } else {
        -1
    };

    ret
}

fn ch_card(ch_count: i32, current: usize) -> i32 {
    let r = vec![5, 15, 25, 35];
    let u = vec![12, 28];

    let ret = if ch_count == 0 {
        GO as i32
    } else if ch_count == 1 {
        JAIL as i32
    } else if ch_count == 2 {
        11
    } else if ch_count == 3 {
        24
    } else if ch_count == 4 {
        39
    } else if ch_count == 5 {
        5
    } else if ch_count == 6 || ch_count == 7 {
        let next = r.iter().filter(|n| **n > current as i32).min();
        if next.is_none() {
            5
        } else {
            *next.unwrap()
        }
    } else if ch_count == 8 {
        let next = u.iter().filter(|n| **n > current as i32).min();
        if next.is_none() {
            12
        } else {
            *next.unwrap()
        }
    } else if ch_count == 9 {
        if current as i32 - 3 < 0 {
            40 - (current as i32 - 3)
        } else {
            (current - 3) as i32
        }
    } else {
        -1
    };

    ret
}

fn main() {
    //CCのマス目
    let cc: Vec<usize> = vec![2, 17, 33];
    //CHのマス目
    let ch: Vec<usize> = vec![7, 22, 36];

    let mut rng = rand::thread_rng();
    let mut cells = [0; 40];
    let mut current = 0;
    let mut same_dice_count = 0;
    let mut cc_count: i32 = 0;
    let mut ch_count: i32 = 0;

    for _ in 0..LOOP {
        let (same_dice, mut dice_val) = dice(&mut rng);

        //ゾロ目
        if same_dice {
            same_dice_count += 1;
            //3回連続ゾロ目
            if same_dice_count == 3 {
                same_dice_count = 0;
                dice_val = 0;
                current = JAIL;
            }
        } else {
            same_dice_count = 0;
        }

        //現在位置
        current += dice_val as usize;

        let mut break_flg = false;
        while !break_flg {
            break_flg = true;

            //配列を超えた
            if current >= 40 {
                current -= 40;
            } else if current == G2JAIL {
                current = JAIL;
            }

            //ccマスの時
            if cc.iter().any(|n| *n == current) {
                let force_cell = cc_card(cc_count);
                cc_count += 1; 
                if cc_count >= 16 {
                    cc_count = 0;
                } 

                if force_cell != -1 {
                    current = force_cell as usize;
                    break_flg = false;
                }
            }

            //chマスの時
            if ch.iter().any(|n| *n == current) {
                let force_cell = ch_card(ch_count, current);
                ch_count += 1;
                if ch_count >= 16 {
                    ch_count = 0;
                }

                if force_cell != -1 {
                    current = force_cell as usize;
                    break_flg = false;
                }
            }
        }

        //止まった位置の更新        
        cells[current as usize] += 1;
    }


    let mut result = cells.iter()
        .enumerate()
        .map(|(i, cell)| Answer::new(i, *cell))
        .collect::<Vec<Answer>>();
    
    result.sort_by(|ans1, ans2| 
        if ans1.value < ans2.value {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        });
    
    result.iter()
        .take(6)
        .for_each(|ans| println!("{:?}", ans));
}

#[derive(Debug)]
struct Answer {
    pub index: usize,
    pub value: f64,
}

impl Answer {
    pub fn new(index: usize, value: i32) -> Self {
        Self {
            index,
            value: value as f64 / LOOP as f64
        }
    }
}