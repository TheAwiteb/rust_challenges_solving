// https://www.codewars.com/kata/5539fecef69c483c5a000015/rust

fn is_prime(num: u64) -> bool {
    for n in 2..if num > 10000 { 10000 } else { num } {
        if num % n == 0 {
            return false;
        }
    }
    true
}

fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut num_list = Vec::<u64>::new();
    for num in start..=stop {
        let back_word_num: u64 = num
            .to_string()
            .chars()
            .rev()
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();
        if (num != back_word_num) && (is_prime(num)) && (is_prime(back_word_num)) {
            num_list.push(num);
        }
    }
    num_list
}

// OR


fn _is_prime(num: u64) -> bool {
    (2..(num as f64).sqrt() as u64 + 1).all(|n| num % n != 0)
}

fn _backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    (start..=stop)
        .filter(|&n| _is_prime(n))
        .filter(|&n| {
            let back_word_num: u64 = n
                .to_string()
                .chars()
                .rev()
                .into_iter()
                .collect::<String>()
                .parse()
                .unwrap();
            back_word_num != n && _is_prime(back_word_num)
        })
        .collect()
}

fn main() {
    tests();
}

fn testing(start: u64, stop: u64, exp: Vec<u64>) -> () {
    assert_eq!(backwards_prime(start, stop), exp)
}

fn tests() {
    let a = vec![13, 17, 31, 37, 71, 73, 79, 97];
    testing(1, 100, a);
    let a = vec![13, 17, 31];
    testing(1, 31, a);
}
