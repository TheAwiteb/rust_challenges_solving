// https://www.codewars.com/kata/59a8570b570190d313000037/rust

fn sum_cubes(n: u32) -> u32 {
    (0..=n).reduce(|total, num| total + (num.pow(3))).unwrap()
}
fn main() {
    tests()
}

fn tests() {
    assert_eq!(sum_cubes(1), 1);
    assert_eq!(sum_cubes(2), 9);
    assert_eq!(sum_cubes(3), 36);
    assert_eq!(sum_cubes(4), 100);
    assert_eq!(sum_cubes(10), 3_025);
    assert_eq!(sum_cubes(123), 58_155_876);
}
