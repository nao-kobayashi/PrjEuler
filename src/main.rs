#[allow(dead_code)]

use std::time::Instant;
fn problem1() {
    println!("{}",
        (0..1000)
            .filter(|i| i % 3 == 0 || i % 5 == 0)
            .sum::<i32>()
    );
}

fn problem2(a: i32, b: i32, mut sum: i32) {
    let c = a + b;

    if b % 2 == 0 {
        sum += b;
    }

    if c >= 4000000 {
        println!("answer:{}", sum);
    }else {
        problem2(b, c, sum);
    }
}

fn problem3(n: f64) {
    let s = n.sqrt() as usize;
    let mut y = n as i64;

    for i in 2..s + 1 {
        while y % (i as i64) == 0 {
            println!("{}", i);
            y = y / i as i64;
        }
    }
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

fn problem4() {
//    let mut result = Vec::new();
//    for x in 100..1000 {
//        for y in 100..1000 {
//            let ans = x * y;
//            if is_palindrome(&ans.to_string()) {
//                result.push(ans);
//            }
//        }
//    }
//
//    println!("{}", result.iter().map(|n| *n).max().unwrap());

    println!("{}",
             (100..1000)
                .map(|x|
                    (100..1000)
                        .filter_map(|y|{
                            let n = x * y;
                            if is_palindrome(&n.to_string()) {
                                Some(n)
                            } else {
                                None
                            }
                        })
                        .max()
                        .unwrap_or(-1)
                )
                .max()
                .unwrap()
    );
}

use std::cmp::{ max, min };
fn gcd(a: i64, b: i64) -> i64 {
    let max = max(a, b);
    let min = min(a, b);

    if max % min == 0 {
        min
    } else {
        gcd(min, max % min)
    }
}

fn problem5(a: i64, b: i64, max: i64) {
    let c = a * b / gcd(a, b);

    if b == max {
        println!("{}", c);
        return;
    }

    problem5(c, b + 1, max);
}

fn problem6(s: i32, e: i32) {
    let a: i32 = (s..e + 1)
        .map(|n| n * n)
        .sum();

    let b: i32 = (s..e + 1)
        .map(|n| n)
        .sum();

    println!("{}", b * b - a);
}

//素数判定
fn is_prime(n: i32) -> bool {
    if n < 2  { return false; }
    else if n == 2 { return true; }
    else if n % 2 == 0 { return false; }

    let sqrt = (n as f64).sqrt() as i32;
    (3..sqrt + 1)
        .filter(|x| x % 2 != 0)
        .all(|x|
            if n % x == 0 {
                false
            } else {
                true
            }
        )
}

fn problem7(pos: i32) {
    let mut num = 0;
    let mut cnt = 0;
    while cnt != pos {
        num += 1;
        if is_prime(num) {
            cnt += 1;
        }
    }
    println!("{}", num);
}

fn problem8(mul_num: usize) {
    let data = "73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";

    //改行コードが来てしまう。
    let num_arr = data
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u64)
        .collect::<Vec<u64>>();

    let mut cur_ans = 0;
    let e = num_arr.len() - mul_num - 1;

    for i in 0..e {
        let mut calc = (0..mul_num).fold(1, |mul, h| mul * num_arr[h + i]);
        cur_ans = max(cur_ans, calc);
    }

    println!("{}", cur_ans);
}

fn problem9(ans: i32) {
    for i in 1..ans {
        for h in i + 1..ans {
            for z in h + 1..ans {
                let p1 = (i as f64).powi(2);
                let p2 = (h as f64).powi(2);
                let p3 = (z as f64).powi(2);

                if p1 + p2 == p3 && i + h + z == ans {
                    println!("{} {} {} {:?} {:?} {:?}", i, h, z, p1, p2, p3);
                    println!("ans {}*{}*{}={}", i, h, z, i * h* z);
                    return;
                }
            }
        }
    }
}

fn problem10(max: i32) {
    let ans: i64 = (2..max + 1)
        .filter(|n| is_prime(*n))
        .map(|n| n as i64)
        .sum();

    println!("{}", ans);
}

fn problem11() {
    let data = [[08,02,22,97,38,15,00,40,00,75,04,05,07,78,52,12,50,77,91,08],
        [49,49,99,40,17,81,18,57,60,87,17,40,98,43,69,48,04,56,62,00],
        [81,49,31,73,55,79,14,29,93,71,40,67,53,88,30,03,49,13,36,65],
        [52,70,95,23,04,60,11,42,69,24,68,56,01,32,56,71,37,02,36,91],
        [22,31,16,71,51,67,63,89,41,92,36,54,22,40,40,28,66,33,13,80],
        [24,47,32,60,99,03,45,02,44,75,33,53,78,36,84,20,35,17,12,50],
        [32,98,81,28,64,23,67,10,26,38,40,67,59,54,70,66,18,38,64,70],
        [67,26,20,68,02,62,12,20,95,63,94,39,63,08,40,91,66,49,94,21],
        [24,55,58,05,66,73,99,26,97,17,78,78,96,83,14,88,34,89,63,72],
        [21,36,23,09,75,00,76,44,20,45,35,14,00,61,33,97,34,31,33,95],
        [78,17,53,28,22,75,31,67,15,94,03,80,04,62,16,14,09,53,56,92],
        [16,39,05,42,96,35,31,47,55,58,88,24,00,17,54,24,36,29,85,57],
        [86,56,00,48,35,71,89,07,05,44,44,37,44,60,21,58,51,54,17,58],
        [19,80,81,68,05,94,47,69,28,73,92,13,86,52,17,77,04,89,55,40],
        [04,52,08,83,97,35,99,16,07,97,57,32,16,26,26,79,33,27,98,66],
        [88,36,68,87,57,62,20,72,03,46,33,67,46,55,12,32,63,93,53,69],
        [04,42,16,73,38,25,39,11,24,94,72,18,08,46,29,32,40,62,76,36],
        [20,69,36,41,72,30,23,88,34,62,99,69,82,67,59,85,74,04,36,16],
        [20,73,35,29,78,31,90,01,74,31,49,71,48,86,81,16,23,57,05,54],
        [01,70,54,71,83,51,54,69,16,92,33,48,61,43,52,01,89,19,67,48]
    ];

    let mut values = Vec::new();
    for x in 0..20 {
        for y in 0..20 {
            if x < 17 as usize {
                let h1 = data[y][x];
                let h2 = data[y][x + 1];
                let h3 = data[y][x + 2];
                let h4 = data[y][x + 3];
                values.push(h1 * h2 * h3 * h4);
            }

            if x < 17 as usize && y < 17 as usize {
                let c_r1 = data[y][x];
                let c_r2 = data[y + 1][x + 1];
                let c_r3 = data[y + 2][x + 2];
                let c_r4 = data[y + 3][x + 3];
                values.push(c_r1 * c_r2 * c_r3 * c_r4);
            }

            if y < 17 as usize {
                let v1 = data[x][y];
                let v2 = data[x][y + 1];
                let v3 = data[x][y + 2];
                let v4 = data[x][y + 3];
                values.push(v1 * v2 * v3 * v4);
            }

            if x > 2 as usize && y < 17 as usize {
                let c_l1 = data[y][x];
                let c_l2 = data[y + 1][x - 1];
                let c_l3 = data[y + 2][x - 2];
                let c_l4 = data[y + 3][x - 3];
                values.push(c_l1 * c_l2 * c_l3 * c_l4);
            }
        }
    }

    println!("{}", values.iter().map(|n| n).max().unwrap());
}

//素因数分解
fn factoring(n: f64) -> Vec<(i64, i32)> {
    if n == 1.0 {
        return vec![(1, 1)];
    }

    let s = n.sqrt() as usize;
    let mut y = n as i64;
    let mut r = 0;

    let mut result = Vec::new();

    for x in 2..s + 1 {
        if y % (x as i64) == 0 {
            r = 0;
            while y % (x as i64) == 0 {
                r += 1;
                y = y / x as i64;
            }
            result.push((x as i64, r));
        }
    }

    if y as usize > s  {
        result.push((y as i64, 1));
    }

    result
}

fn problem12(target: i32) {
    let ans = (1..std::i32::MAX)
        .find_map(|n| {
            let tr: i32 = (1..n + 1).map(|t| t).sum();

            //素因数分解した結果の指数部それぞれに1足して全部掛け合わせると約数の数になる
            let ans = factoring(tr as f64)
                .iter()
                .fold(1, |calc, (num, r)| calc * (r + 1));

            if ans > target {
                Some(tr)
            } else {
                None
            }
        });

    println!("{:?}", ans.unwrap_or(-1));
}

extern crate fnv;
use fnv::{ FnvHashMap, FnvHashSet };
fn problem14() {
    let mut ans = 0;
    let mut max_num = 0;
    let mut cache = FnvHashMap::default();

    for i in 2..1000000 {
        let mut ret: i64 = i;
        let mut count: i64 = 1;

        while ret != 1 {
            ret = if ret % 2 == 0 {
                ret / 2
            } else {
                ret * 3 + 1
            };

            if cache.contains_key(&ret) {
//                println!("cache hit loop in {} at {}", i, ret);
                count += cache[&ret];
                break;
            }
            count += 1;
        }

        max_num = max(max_num, count);
        if count == max_num {
            ans = i;
        }

        //println!("{} {}", i, count);
        cache.insert(i, count);
    }

    println!("answer:{}", ans);
}

extern crate num_bigint;
use num_bigint:: BigInt;
fn problem13() {
    let data = ["37107287533902102798797998220837590246510135740250",
        "46376937677490009712648124896970078050417018260538",
        "74324986199524741059474233309513058123726617309629",
        "91942213363574161572522430563301811072406154908250",
        "23067588207539346171171980310421047513778063246676",
        "89261670696623633820136378418383684178734361726757",
        "28112879812849979408065481931592621691275889832738",
        "44274228917432520321923589422876796487670272189318",
        "47451445736001306439091167216856844588711603153276",
        "70386486105843025439939619828917593665686757934951",
        "62176457141856560629502157223196586755079324193331",
        "64906352462741904929101432445813822663347944758178",
        "92575867718337217661963751590579239728245598838407",
        "58203565325359399008402633568948830189458628227828",
        "80181199384826282014278194139940567587151170094390",
        "35398664372827112653829987240784473053190104293586",
        "86515506006295864861532075273371959191420517255829",
        "71693888707715466499115593487603532921714970056938",
        "54370070576826684624621495650076471787294438377604",
        "53282654108756828443191190634694037855217779295145",
        "36123272525000296071075082563815656710885258350721",
        "45876576172410976447339110607218265236877223636045",
        "17423706905851860660448207621209813287860733969412",
        "81142660418086830619328460811191061556940512689692",
        "51934325451728388641918047049293215058642563049483",
        "62467221648435076201727918039944693004732956340691",
        "15732444386908125794514089057706229429197107928209",
        "55037687525678773091862540744969844508330393682126",
        "18336384825330154686196124348767681297534375946515",
        "80386287592878490201521685554828717201219257766954",
        "78182833757993103614740356856449095527097864797581",
        "16726320100436897842553539920931837441497806860984",
        "48403098129077791799088218795327364475675590848030",
        "87086987551392711854517078544161852424320693150332",
        "59959406895756536782107074926966537676326235447210",
        "69793950679652694742597709739166693763042633987085",
        "41052684708299085211399427365734116182760315001271",
        "65378607361501080857009149939512557028198746004375",
        "35829035317434717326932123578154982629742552737307",
        "94953759765105305946966067683156574377167401875275",
        "88902802571733229619176668713819931811048770190271",
        "25267680276078003013678680992525463401061632866526",
        "36270218540497705585629946580636237993140746255962",
        "24074486908231174977792365466257246923322810917141",
        "91430288197103288597806669760892938638285025333403",
        "34413065578016127815921815005561868836468420090470",
        "23053081172816430487623791969842487255036638784583",
        "11487696932154902810424020138335124462181441773470",
        "63783299490636259666498587618221225225512486764533",
        "67720186971698544312419572409913959008952310058822",
        "95548255300263520781532296796249481641953868218774",
        "76085327132285723110424803456124867697064507995236",
        "37774242535411291684276865538926205024910326572967",
        "23701913275725675285653248258265463092207058596522",
        "29798860272258331913126375147341994889534765745501",
        "18495701454879288984856827726077713721403798879715",
        "38298203783031473527721580348144513491373226651381",
        "34829543829199918180278916522431027392251122869539",
        "40957953066405232632538044100059654939159879593635",
        "29746152185502371307642255121183693803580388584903",
        "41698116222072977186158236678424689157993532961922",
        "62467957194401269043877107275048102390895523597457",
        "23189706772547915061505504953922979530901129967519",
        "86188088225875314529584099251203829009407770775672",
        "11306739708304724483816533873502340845647058077308",
        "82959174767140363198008187129011875491310547126581",
        "97623331044818386269515456334926366572897563400500",
        "42846280183517070527831839425882145521227251250327",
        "55121603546981200581762165212827652751691296897789",
        "32238195734329339946437501907836945765883352399886",
        "75506164965184775180738168837861091527357929701337",
        "62177842752192623401942399639168044983993173312731",
        "32924185707147349566916674687634660915035914677504",
        "99518671430235219628894890102423325116913619626622",
        "73267460800591547471830798392868535206946944540724",
        "76841822524674417161514036427982273348055556214818",
        "97142617910342598647204516893989422179826088076852",
        "87783646182799346313767754307809363333018982642090",
        "10848802521674670883215120185883543223812876952786",
        "71329612474782464538636993009049310363619763878039",
        "62184073572399794223406235393808339651327408011116",
        "66627891981488087797941876876144230030984490851411",
        "60661826293682836764744779239180335110989069790714",
        "85786944089552990653640447425576083659976645795096",
        "66024396409905389607120198219976047599490197230297",
        "64913982680032973156037120041377903785566085089252",
        "16730939319872750275468906903707539413042652315011",
        "94809377245048795150954100921645863754710598436791",
        "78639167021187492431995700641917969777599028300699",
        "15368713711936614952811305876380278410754449733078",
        "40789923115535562561142322423255033685442488917353",
        "44889911501440648020369068063960672322193204149535",
        "41503128880339536053299340368006977710650566631954",
        "81234880673210146739058568557934581403627822703280",
        "82616570773948327592232845941706525094512325230608",
        "22918802058777319719839450180888072429661980811197",
        "77158542502016545090413245809786882778948721859617",
        "72107838435069186155435662884062257473692284509516",
        "20849603980134001723930671666823555245252804609722",
        "53503534226472524250874054075591789781264330331690"];

    let sum_bigint = data.iter()
        .map(|s| BigInt::parse_bytes(s.as_bytes(), 10).unwrap())
        .sum::<BigInt>();

    println!("{}", &sum_bigint.to_string()[..10]);

}

fn problem15(size: usize) {
    let row_size = size + 1;
    let capacity = (size + 1) * (size + 1);
    let mut grid: Vec<i64> = Vec::with_capacity(capacity);

    //init vector
    for i in 1..capacity + 1 {
        if i % (size + 1) == 0 {
            grid.push(1);
        } else if i >= capacity - (size + 1) {
            grid.push(1);
        } else {
            grid.push(0);
        }
    }

//    println!("{:?}", grid);
    let mut y = size - 1;
    while y >= 0 {
        let mut x = size - 1;
        while x >= 0 {
            //y is row. it has size + 1 columns.
            *grid.get_mut((y * row_size) + x).unwrap() = &grid[(y + 1) * row_size + x] + &grid[(y * row_size) + x + 1];
            if x == 0 { break; }
            x -= 1;
        }

        if y == 0 { break; }
        y -= 1;
    }

    println!("{:?}", grid);
}

fn problem16() {
    let work = BigInt::parse_bytes("1".as_bytes(), 10).unwrap();
    let big_int = (0..1000)
        .fold(work,|a, _b| a * BigInt::parse_bytes("2".as_bytes(), 10).unwrap());

    println!("{}", big_int.to_string());
    let ans = big_int.to_string().chars().map(|n| n.to_digit(10).unwrap() as u64).sum::<u64>();
    println!("{:?}", ans);

}

#[macro_use]
extern crate lazy_static;
pub mod problem17;
use crate::problem17::*;

fn problem18() {
    let data = vec![vec![75],
                    vec![95,64],
                    vec![17,47,82],
                    vec![18,35,87,10],
                    vec![20,04,82,47,65],
                    vec![19,01,23,75,03,34],
                    vec![88,02,77,73,07,63,67],
                    vec![99,65,04,28,06,16,70,92],
                    vec![41,41,26,56,83,40,80,70,33],
                    vec![41,48,72,33,47,32,37,16,94,29],
                    vec![53,71,44,65,25,43,91,52,97,51,14],
                    vec![70,11,33,28,77,73,17,78,39,68,17,57],
                    vec![91,71,52,38,17,14,91,43,58,50,27,29,48],
                    vec![63,66,04,68,89,53,67,30,73,16,69,87,40,31],
                    vec![04,62,98,27,23,09,70,98,73,93,38,53,60,04,23]
    ];
    let mut index = data.len() - 1;
    let mut calculated = Vec::new();

    while index > 0 {
        let from_data = if calculated.len() == 0 {
            data[index].clone()
        } else {
            calculated.clone()
        };
        calculated.clear();

        for (i, n) in data[index - 1].iter().enumerate() {
            calculated.push(n + max(from_data[i], from_data[i + 1]));
        }

        index -= 1;
    }

    println!("{:?}", calculated);

}

fn problem19() {
    let mut count = 0;
    let mut day_of_week = 2;
    let months = [31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for y in 1901..2001 {
        for (i, m) in months.iter().enumerate() {
            let days = if i == 1 {
                if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
                    29
                } else {
                    28
                }
            } else {
                *m
            };

            //calc first day of next month.
            day_of_week += days % 7;
            if day_of_week >= 7 {
                day_of_week -= 7;
            }
            println!("{} {} {}", y, i + 2, day_of_week);

            //for last loop calc day of week at 2001.01
            if day_of_week == 0 {
                if !(y == 2000 && i == 11) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn problem20() {
    let a = BigInt::parse_bytes("1".as_bytes(), 10).unwrap();
    let ans = (1..101)
        .fold(a, |x, y| x * BigInt::parse_bytes(&y.to_string().as_bytes(), 10).unwrap());
    let num = ans.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();


    println!("{}", ans.to_string());
    println!("{}", num);
}

fn divisors(n: i32) -> Vec<i32> {
    (1..n + 1)
        .filter_map(|d|{
            if n % d == 0 {
                Some(d)
            } else {
                None
            }
        })
        .collect::<Vec<i32>>()
}

fn problem21() {
    let mut hash = FnvHashMap::default();

    for i in 1..100000 {
        let mut devisor = divisors(i);
        devisor.pop().unwrap();
        //println!("{}:{:?}", i, devisor);
        hash.insert(i, devisor.iter().map(|n| *n).sum::<i32>());
    }

    //println!("{:?}", hash[&2]);
    let ans = (1..100000)
        .map(|n|{
            if hash.contains_key(&n) && hash.contains_key(&hash[&n]) && &hash[&n] != &n && hash[&hash[&n]] == n {
                //println!("{} {}", n, hash[&hash[&n]]);
                n + hash[&hash[&n]]
            }
            else {
                0
            }
        })
        .collect::<Vec<i32>>();

    println!("{}", ans.iter().map(|n| * n).sum::<i32>() / 2);
}

//約数の合計を返す。
//220 => 素因数分解 2^2, 5, 11
//(1 + 2 + 2^2) * (1 + 5) * (1 + 11)
//= 504 = 504 - 220 = 284
//504が本来の約数の合計、284は自分自身を引いた約数の合計
fn divisors_sum(n: i32, minus: i32) -> i32 {
    let a = factoring(n as f64).iter()
        .map(|(f, r)|{
            (1..r + 1)
                .map(|i| (*f as f64).powi(i) as i64)
                .sum::<i64>() + 1
        })
        .collect::<Vec<i64>>();

    //println!("{:?}", a);
    let b = a.iter().fold(1_i64, |x, y | x * y);
    //minus own.
    b as i32 - minus
}

fn problem21a() {
    let mut ans = 0;
    let mut hash = FnvHashSet::default();

    for i in 1..100000 {
        if hash.contains(&i) { continue; }

        let mut devisor = divisors(i);
        devisor.pop().unwrap();
        if devisor.len() == 0 { continue; }
        let devisor_sum =  devisor.iter().map(|n| *n).sum::<i32>();
        if i == devisor_sum  { continue; }


        let mut devisor2 = divisors(devisor_sum);
        devisor2.pop().unwrap();
        if devisor2.len() == 0 { continue; }
        let devisor_sum2 =  devisor2.iter().map(|n| *n).sum::<i32>();
        if devisor_sum2 == i  {
            //println!("{} {} {}", i, devisor_sum2, devisor_sum);
            ans += i + devisor_sum;
            hash.insert(i);
            hash.insert(devisor_sum);
        }
    }

    println!("{}", ans);
}

fn problem21b() {
    let mut hash = FnvHashSet::default();

    for i in 1..100000 {
        let devisor_sum =  divisors_sum(i, i);
        //println!("{} {} ", i, i_devisor_sum);
        if i == devisor_sum  { continue; }

        let devisor_sum2 =  divisors_sum(devisor_sum, devisor_sum);
        if devisor_sum2 == i  {
            if hash.contains(&i) || hash.contains(&devisor_sum) { continue; }
            //println!("{} {} {}", i, devisor_sum2, i_devisor_sum);
            hash.insert(i);
            hash.insert(devisor_sum);
        }
    }

    println!("{}", hash.iter().map(|n| *n).sum::<i32>());
}

use std::collections::HashSet;

const BUF_SIZE: usize = 2048;

fn problem22() {
    let mut points = FnvHashMap::default();
    points.insert("A".to_string(), 1);
    points.insert("B".to_string(), 2);
    points.insert("C".to_string(), 3);
    points.insert("D".to_string(), 4);
    points.insert("E".to_string(), 5);
    points.insert("F".to_string(), 6);
    points.insert("G".to_string(), 7);
    points.insert("H".to_string(), 8);
    points.insert("I".to_string(), 9);
    points.insert("J".to_string(), 10);
    points.insert("K".to_string(), 11);
    points.insert("L".to_string(), 12);
    points.insert("M".to_string(), 13);
    points.insert("N".to_string(), 14);
    points.insert("O".to_string(), 15);
    points.insert("P".to_string(), 16);
    points.insert("Q".to_string(), 17);
    points.insert("R".to_string(), 18);
    points.insert("S".to_string(), 19);
    points.insert("T".to_string(), 20);
    points.insert("U".to_string(), 21);
    points.insert("V".to_string(), 22);
    points.insert("W".to_string(), 23);
    points.insert("X".to_string(), 24);
    points.insert("Y".to_string(), 25);
    points.insert("Z".to_string(), 26);

    let text_contents = std::fs::read_to_string("./names.txt").unwrap();
    let replace_string = text_contents.replace("\"", "");
    let mut names = replace_string.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
    names.sort();

    let mut sum = 0;
    for (i, s) in names.iter().enumerate() {
        sum += s.chars()
            .map(|c| points[&c.to_string()])
            .sum::<i32>() * ((i as i32) + 1);
    }

    println!("{:?}", sum);
}

fn problem23() {
    let v = (1..28124)
        .filter_map(|n|
            if divisors_sum(n, n) > n {
                Some(n)
            } else {
                None
            }
        )
        .collect::<Vec<i32>>();

    let mut hash = FnvHashSet::default();
    v.iter()
        .for_each(|a| {
            v.iter()
                .for_each(|b| {
                    hash.insert(a + b);
                })
        });

    let ans = (1..28124)
        .filter_map(|n| {
            if !hash.contains(&n) {
                Some(n)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("{}", ans);
}

fn problem24(mut num: i32) {
    let mut ans = String::new();
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut m = num;

    let max = (1..11).fold(1, |a, b| a * b);
    if max < num { return; }

    while v.len() > 0 {
        let len = v.len() as i32;
        let combi = (1..len).fold(1, |a, b| a * b);

        if m == 0 {
            ans += &v.pop().unwrap().to_string();
            break;
        }

        let d = m / combi;
        m = m % combi;
        if m != 0 {
            ans += &v.get(d as usize).unwrap().to_string();
            v.remove(d as usize);
        } else {
            ans += &v.get((d - 1) as usize).unwrap().to_string();
            v.remove((d - 1) as usize);
        }
    }

    println!("{}", ans + &v.get(0).unwrap().to_string());
}

fn fibonacci(max_len: usize) {
    let a = BigInt::from(1);
    let mut b = BigInt::from(1);
    let mut c = BigInt::from(2);
    let mut count = 3;

    loop {
        if c.to_string().len() >= max_len {
            println!("No.{} {:?}", count, c);
            break;
        }else {
            let d = c.clone();
            c = b + c;
            b = d;
            count += 1;
        }
    }
}

fn problem25() {
    fibonacci(1000);
}

fn problem26(num: i32) {
    let mut ans = 0;
    let primes = (1..num).filter(|n| n != &2 && n != &5 && is_prime(*n)).collect::<Vec<i32>>();

    for p in primes {
        let mut i = 1;
        let mut remainder = 10;
        let mut remainder_list = Vec::new();

        loop {
            remainder = remainder % p;
            if remainder_list.iter().any(|n| n == &remainder) {
                break;
            }

            remainder_list.push(remainder);
            i += 1;
            remainder = remainder * 10;
        }

        let len = i - (remainder_list[remainder_list.iter().position(|n| n == &remainder).unwrap()] + 1);
        if ans < len {
            println!("{} {:?}", p, remainder_list);
            ans = p;
        }
    }

    println!("{}", ans);
}

fn problem27() {
    let mut max_comb = (0,0);
    let mut max_count = 0;

    for a in -999..1000 {
        for b in -999..1000 {
            if is_prime(b) && is_prime(1 + a + b) {
                let mut n = 0;
                let mut count = 0;
                loop {
                    if is_prime((n * n) + (a * n) + b) {
                        count += 1;
                    } else {
                        if max_count < count {
                            max_count = count;
                            max_comb = (a, b);
                        }
                        break;
                    }
                    n += 1;
                }
            }
        }
    }
    println!("a, b:{:?} = {}.  max_count:{}", max_comb, max_comb.0 * max_comb.1 ,max_count);
}

fn problem28() {
    let mut base: u64 = 3;
    let mut c1: u64 = 0;
    let mut c2: u64 = 0;
    let mut c3: u64 = 0;
    let mut c4: u64 = 0;
    let mut ans: u64 = 0;
    let limit: u64 = 1001 * 1001;

    loop {
        c1 = base * base;
        c2 = c1 - base + 1;
        c3 = c2 - base + 1;
        c4 = c3 - base + 1;
        ans += c1 + c2 + c3 + c4;

        base += 2;
        if c1 == limit { break; }
    }

    println!("ans {}", ans + 1);

}

use std::ops::Mul;

fn problem29() {
    let mut hash = HashSet::new();

    let a = (2..101)
        .map(|n| n)
        .collect::<Vec<i32>>();

    let b = (2..101)
        .map(|n| n)
        .collect::<Vec<i32>>();

    for na in a.iter() {
        for nb in b.iter() {
            let mut tmp = BigInt::from(*na);
            for i in 1..*nb {
                tmp = tmp.mul(na);
            }
            hash.insert(tmp);
        }
    }

    println!("{}", hash.len());
}

fn problem30_test() {
    for i in 1..10 {
        let a = (10.0 as f64).powi(i - 1);
        let b = (9.0 as f64).powi(5) * i as f64;

        println!("a:{} b:{} i:{} a <= b:{}", a, b, i, a<=b);
    }
}

fn problem30() {
    let mut ans = 0;
    //problem30_test()より6桁内に最大値があることが分かる。
    //7桁になると5乗では到達しない。
    //9^6 + 1 = 354295
    for n in 2..354295 {
        let sum = n.to_string()
            .chars()
            .map(|n| f64::from(n.to_digit(10).unwrap()).powi(5) as i32)
            .sum::<i32>();

        if n == sum {
            println!("found number is {}.", sum);
            ans += n;
        }
    }

    println!("ans:{}", ans);
}

fn main() {
    let start = Instant::now();
    problem1();
    problem2(1, 2, 0);
    problem3(600851475143.0);
    println!("{}", is_palindrome("abcd"));
    println!("{}", is_palindrome("abba"));
    problem4();
    problem5(1, 2, 11);
    problem6(1, 100);

    println!("{}", is_prime(19));
    problem7(10001);
    problem8(13);
    problem9(1000);
    problem10(2000000);
    problem11();
    problem12(500);
    problem13();
    problem14();
    problem15(20);
    problem16();

    problem17::solve();
    problem18();
    problem19();
    problem20();

    println!("{:?}", divisors(220));
    println!("{:?}", factoring(25.0));
    println!("{:?}", divisors_sum(25, 25));

    problem21();
    problem21a();
    problem21b();
    problem22();
    problem23();
    problem24(1000000);
    problem25();
    problem26(1000);
    problem27();
    problem28();
    problem29();
    problem30_test();
    problem30();

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

}
