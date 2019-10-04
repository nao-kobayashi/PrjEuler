use std::time::Instant;

struct Pattern<'a, T> {
    pub patterns: Vec<Vec<T>>,
    pub elements: &'a [T],
}

impl<'a, T> Pattern<'a, T> {
    pub fn new(elements: &'a [T]) -> Self {
        Self {
            patterns: Vec::new(),
            elements,
        }
    }

    fn set_next(&self, index: usize, elements: &[T], vec: &mut Vec<T>)
    where
        T: Clone,
    {
        for (i, n) in elements.iter().enumerate() {
            if i != index {
                vec.push((*n).clone());
            }
        }
    }

    pub fn create(&mut self)
    where
        T: PartialEq,
        T: Clone,
        T: PartialOrd,
    {
        let mut work = Vec::new();
        self.create_formular_pattern(self.elements, &mut work);
        self.patterns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.patterns.dedup();
    }

    fn create_formular_pattern(&mut self, elements: &[T], result: &mut Vec<T>)
    where
        T: Clone,
    {
        if elements.len() == 1 {
            result.push(elements[0].clone());
            self.patterns.push(result.clone());
            let _ = result.pop();
        } else {
            for i in 0..elements.len() {
                let mut next_elements = Vec::new();
                self.set_next(i, elements, &mut next_elements);

                result.push(elements[i].clone());
                self.create_formular_pattern(&next_elements, result);
                let _ = result.pop();
            }
        }
    }
}

fn calc(arr: &[String]) -> Result<i32, i32> {
    let mut ans = 0.0;
    let mut ans_stack = Vec::new();

    for c in arr {
        if c == "+" || c == "-" || c == "*" || c == "/" {
            if ans_stack.len() < 2 {
                return Err(0);
            }
            let back = ans_stack.pop().unwrap();
            let front = ans_stack.pop().unwrap();

            if c == "+" {
                ans = front + back;
            } else if c == "-" {
                ans = front - back;
            } else if c == "*" {
                ans = front * back;
            } else if c == "/" {
                if back == 0.0 {
                    return Err(1);
                }
                ans = front / back;
            }

            ans_stack.push(ans);
        } else {
            ans_stack.push(c.parse::<f64>().unwrap());
        }
    }

    if (ans as i32) as f64 == ans {
        Ok(ans as i32)
    } else {
        Ok(0)
    }
}

fn create_operators() -> Vec<Vec<String>> {
    let operators = vec!["+", "-", "*", "/"];
    let mut ope_result = Vec::new();

    for o1 in &operators {
        for o2 in &operators {
            for o3 in &operators {
                for o4 in &operators {
                    ope_result.push(vec![
                        o1.to_string(),
                        o2.to_string(),
                        o3.to_string(),
                        o4.to_string()
                    ]);
                }
            }
        }
    }

    ope_result
}

fn create_source_nums() -> Vec<Vec<f64>> {
    let mut result = Vec::new();
    // result.push(vec![1.0, 2.0, 3.0, 4.0]);

    for n1 in 1..=6 {
        for n2 in n1 + 1..=7 {
            for n3 in n2 + 1 ..=8 {
                for n4 in n3 + 1 ..=9 {
                    result.push(vec![n1 as f64, n2 as f64, n3 as f64, n4 as f64]);
                }
            }
        }
    }

    result
}

fn main() {
    let start = Instant::now();

    //1が数字、0が演算子
    let formular = vec![1, 1, 1, 1, 0, 0, 0];
    let mut form_pat = Pattern::new(&formular);
    form_pat.create();
    //最低限、最初の２つは数字の必要がある。
    let target_formular = form_pat.patterns.iter()
        .filter(|arr| arr[0] == 1 && arr[1] == 1)
        .map(|arr| arr)
        .collect::<Vec<&Vec<i32>>>();

    let all_numbers = create_source_nums();
    let operators = create_operators();

    let mut max_num = 0;
    let mut max_comb = Vec::new();

    for comb in all_numbers {
        let mut numb_pat = Pattern::new(&comb);
        numb_pat.create();

        let mut answer = Vec::new();
        for num_source in &numb_pat.patterns {
            for ope_source in &operators {
                for f_source in &target_formular {
                    let mut ope_index = 0;
                    let mut num_index = 0;

                    let mut stack = Vec::new();
                    for elm in f_source.into_iter() {
                        if *elm == 1 {
                            stack.push(num_source[num_index].to_string());
                            num_index += 1;
                        } else if *elm == 0 {
                            stack.push(ope_source[ope_index].to_string());
                            ope_index += 1;
                        }
                    }

                    match calc(&stack) {
                        Ok(n) => {
                            if n > 0 {
                                answer.push(n);
                            }
                        },
                        Err(e) => {
                            if e == 0 {
                                continue
                            }
                        },
                    }
                }
            }
        }

        answer.sort_by(|a, b| a.partial_cmp(b).unwrap());
        answer.dedup();

        let mut i = 1;
        for n in &answer {
            if *n != i {
                if max_num < i - 1 {
                    max_num = i - 1;
                    max_comb = comb.clone();
                }
                break;
            }

            i += 1;
        }
    }

    println!("longest number {}", max_num);
    println!("max number {:?}", max_comb);

    let elapsed = start.elapsed();
    println!("{} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
