use std::time::Instant;

fn main() {
    let start = Instant::now();

    let f_three =  |n| n * (n + 1) / 2;
    let f_four = |n| n * n;
    let f_five = |n| n * (3 * n - 1) / 2;
    let f_six = |n| n * (2 * n - 1);
    let f_seven = |n| n * (5 * n - 3) / 2;
    let f_eight = |n| n * (  3 * n - 2 );

    let three = create(Box::new(f_three));
    let four = create(Box::new(f_four));
    let five = create(Box::new(f_five));
    let six = create(Box::new(f_six));
    let seven = create(Box::new(f_seven));
    let eight = create(Box::new(f_eight));

    let source = vec![three, four, five, six, seven, eight];
    let indexes = (0..6).map(|n| n).collect::<Vec<usize>>();

    for (i0, elm0) in indexes.iter().enumerate() {
        let mut cp0 = indexes.clone();
        cp0.remove(i0);

        choose(&mut cp0, &mut vec![*elm0], &source);
    }

 
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

fn choose(vecs: &mut Vec<usize>, indexes: &mut Vec<usize>, source: &Vec<Vec<String>>) {

    if vecs.len() == 1 {
        find(
            source.get(indexes[0]).unwrap(), 
            source.get(indexes[1]).unwrap(), 
            source.get(indexes[2]).unwrap(), 
            source.get(indexes[3]).unwrap(), 
            source.get(indexes[4]).unwrap(), 
            source.get(vecs[0]).unwrap());
    } else {
        for (i, elm) in vecs.iter().enumerate() {
            let mut cp = vecs.clone();
            let mut ind_cp = indexes.clone();
            cp.remove(i);
            ind_cp.push(*elm);
            choose(&mut cp, &mut ind_cp, source);
        }
    }
}

fn find(v1: &Vec<String>, v2: &Vec<String>, v3: &Vec<String>, v4: &Vec<String>, v5: &Vec<String>, v6: &Vec<String>) {
    for n1 in v1.iter() {
        for n2 in v2.iter().filter(|s| n1[2..4] == s[0..2]) {
            for n3 in v3.iter().filter(|s| n2[2..4] == s[0..2]) {
                for n4 in v4.iter().filter(|s| n3[2..4] == s[0..2]) {
                    for n5 in v5.iter().filter(|s| n4[2..4] == s[0..2]) {
                        for n6 in v6.iter().filter(|s| n5[2..4] == s[0..2]) {
                            if n6[2..4] == n1[0..2] {
                                println!("{} {} {} {} {} {} = {}", n1, n2, n3, n4, n5, n6,
                                    n1.parse::<i32>().unwrap() + 
                                    n2.parse::<i32>().unwrap() + 
                                    n3.parse::<i32>().unwrap() + 
                                    n4.parse::<i32>().unwrap() + 
                                    n5.parse::<i32>().unwrap() + 
                                    n6.parse::<i32>().unwrap());
                            }
                        }
                    }
                }
            }
        }
    }
}

fn create(handler: Box<Fn(i32) -> i32>) -> Vec<String> {
    let mut nums = Vec::new();
    let mut n = 1;

    loop {
        let elm = handler(n);
        if elm > 999 && elm < 10000 {
            nums.push(elm.to_string());
        }

        if elm > 10000 {
            break;
        }
        n += 1;
    } 

    nums
}