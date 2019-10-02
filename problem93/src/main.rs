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

fn main() {
    let formular = vec![1, 1, 1, 1, 0, 0, 0];
    let mut pat1 = Pattern::new(&formular);
    pat1.create();
    // println!("{:?}", pat1.patterns);

    let nums = vec![1, 2, 3, 4];
    let mut pat2 = Pattern::new(&nums);
    pat2.create();
    println!("{:?}", pat2.patterns);

    let operators = vec!["+", "-", "*", "/"];
    let mut pat3 = Pattern::new(&operators);
    pat3.create();
    println!("{:?}", pat3.patterns);

    for f in &formular {
        
    }

}
