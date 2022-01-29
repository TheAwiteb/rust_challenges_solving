// https://www.codewars.com/kata/59a8570b570190d313000037/rust

// +---------------------------------------------------------------------------------+
// |                                   Sum of Cubes                                  |
// +----------------------------------------+----------------------------------------+
// |                  Rank                  |                  7 kyu                 |
// +----------------------------------------+----------------------------------------+
// |                  Tags                  | Fundamentals, Control Flow, Basic Lang |
// |                                        |              uage Features             |
// +----------------------------------------+----------------------------------------+
// | Write a function that takes a positive integer n, sums all the cubed values fro |
// | m 1 to n, and returns that sum.                                                 |
// |                                                                                 |
// | Assume that the input n will always be a positive integer.                      |
// |                                                                                 |
// | Examples: **(Input --> output)**                                                |
// | ```                                                                             |
// | 2 --> 9 (sum of the cubes of 1 and 2 is 1 + 8)                                  |
// | 3 --> 36 (sum of the cubes of 1, 2, and 3 is 1 + 8 + 27)                        |
// | ```                                                                             |
// +---------------------------------------------------------------------------------+

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
