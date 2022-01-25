// https://www.codewars.com/kata/58f5c63f1e26ecda7e000029/rust

/// Returns Mexican Wave of sentence
///
/// ## Example:
/// ```rust
/// println!("{:#?}", wave("hello, world"));
/// // [
/// //     "Hello, world",
/// //     "hEllo, world",
/// //     "heLlo, world",
/// //     "helLo, world",
/// //     "hellO, world",
/// //     "hello, World",
/// //     "hello, wOrld",
/// //     "hello, woRld",
/// //     "hello, worLd",
/// //     "hello, worlD",
/// // ]
/// println!("{:#?}", wave("hello"));
/// // [
/// // "Hello",
/// // "hEllo",
/// // "heLlo",
/// // "helLo",
/// // "hellO",
/// // ]
/// ```
fn wave(sentence: &str) -> Vec<String> {
    sentence
        .chars()
        .enumerate()
        .filter_map(|(idx, chr)| {
            if !chr.is_alphabetic() {
                None // skip if letter is not alphabetic
            } else {
                Some(format!(
                    "{}{}{}",
                    &sentence[..idx],                       // Before uppercase letter.
                    &sentence[idx..idx + 1].to_uppercase(), // The uppercase letter.
                    &sentence[idx + 1..]                    // After uppercase letter.
                ))
            }
        })
        .collect() // collect to Vec<String>
}

#[test]
fn tests() {
    let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
    assert_eq!(wave("hello"), expect);

    let expect = [
        "Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs",
        "codewarS",
    ];
    assert_eq!(wave("codewars"), expect);

    let expect: [&str; 0] = [];
    assert_eq!(wave(""), expect);

    let expect = [
        "Two words",
        "tWo words",
        "twO words",
        "two Words",
        "two wOrds",
        "two woRds",
        "two worDs",
        "two wordS",
    ];
    assert_eq!(wave("two words"), expect);

    let expect = [" Gap ", " gAp ", " gaP "];
    assert_eq!(wave(" gap "), expect);
}

fn main() {
    println!("hello -> {:#?}", wave("hello"));
    println!("two words -> {:#?}", wave("two words"));
}
