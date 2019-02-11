

lazy_static! {
    // proper nouns
    static ref proper:[usize; 20] = [
    0,
    "one".len(),
    "two".len(),
    "three".len(),
    "four".len(),
    "five".len(),
    "six".len(),
    "seven".len(),
    "eight".len(),
    "nine".len(),
    "ten".len(),
    "eleven".len(),
    "twelve".len(),
    "thirteen".len(),
    "fourteen".len(),
    "fifteen".len(),
    "sixteen".len(),
    "seventeen".len(),
    "eighteen".len(),
    "nineteen".len()
    ];

    // tenth prefixes
    static ref tenth: [usize; 8] = [
    "twenty".len(),
    "thirty".len(),
    "forty".len(),
    "fifty".len(),
    "sixty".len(),
    "seventy".len(),
    "eighty".len(),
    "ninety".len()
    ];
}

pub fn below100(n: i32) -> usize {
    // Returns the len() of the numbers between 0 and 99
    if n < 20 {
        return proper[n as usize];
    }

    tenth[(n / 10 - 2 | 0) as usize] + proper[(n % 10) as usize]
}

pub fn number_length(n: i32) -> usize {
    if n < 100 {
        return below100(n);
    }

    let mut res = 0;
    let h = ((n as f64 / 100.0) % 10.0).floor() as i32;
    let t = (n as f64 / 1000.0).floor() as i32;
    let s = n % 100;

    if n > 999 {
        res += below100(t) + "thousand".len();
    }

    if h != 0 {
        res += proper[h as usize] + "hundred".len();
    }

    if s != 0 {
        res += "and".len() + below100(s);
    }

    res
}

pub fn solve() {
    let ans = (1..1001)
        .map(|n| number_length(n))
        .sum::<usize>();

    println!("{}", ans);
}