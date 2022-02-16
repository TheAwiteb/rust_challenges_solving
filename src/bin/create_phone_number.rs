// https://www.codewars.com/kata/525f50e3b73515a6db000b83/rust

// +---------------------------------------------------------------------------------+
// |                               Create Phone Number                               |
// +----------------------------------------+----------------------------------------+
// |                  Rank                  |                  6 kyu                 |
// +----------------------------------------+----------------------------------------+
// |                  Tags                  | Algorithms, Arrays, Data Types, String |
// |                                        | s, Loops, Control Flow, Basic Language |
// |                                        |  Features, Fundamentals, Formatting, L |
// |                                        | ogic, Regular Expressions, Declarative |
// |                                        |  Programming, Advanced Language Featur |
// |                                        |        es, Programming Paradigms       |
// +----------------------------------------+----------------------------------------+
// | Write a function that accepts an array of 10 integers (between 0 and 9), that r |
// | eturns a string of those numbers in the form of a phone number.                 |
// |                                                                                 |
// | ### Example                                                                     |
// |                                                                                 |
// | ```rust                                                                         |
// | create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"        |
// | ```                                                                            |
// |                                                                                 |
// | The returned format must be correct in order to complete this challenge.        |
// | Don't forget the space after the closing parentheses!                           |
// |                                                                                 |
// +---------------------------------------------------------------------------------+

use std::ops::Range;
/// Convert numbers array to phone number
/// Examples:
/// ```rust
/// assert_eq!(
///     create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
///     "(123) 456-7890"
/// );
///     assert_eq!(
///     create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
/// "(111) 111-1111"
/// );
/// assert_eq!(
///     create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
///     "(123) 456-7899"
/// );
/// ```
fn create_phone_number(numbers: &[u8]) -> String {
    let spliter = |rng: Range<usize>| {
        numbers[rng]
            .into_iter()
            .map(|num: &u8| num.to_string())
            .collect::<Vec<String>>()
            .concat()
    };
    format!("({}) {}-{}", spliter(0..3), spliter(3..6), spliter(6..10))
}

fn main() {
    println!(
        "Output of [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]\n -> {}",
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    )
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
