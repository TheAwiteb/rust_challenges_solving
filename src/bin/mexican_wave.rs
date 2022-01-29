// https://www.codewars.com/kata/58f5c63f1e26ecda7e000029/rust

// +---------------------------------------------------------------------------------+
// |                                   Mexican Wave                                  |
// +----------------------------------------+----------------------------------------+
// |                  Rank                  |                  6 kyu                 |
// +----------------------------------------+----------------------------------------+
// |                  Tags                  | Fundamentals, Arrays, Data Types, Stri |
// |                                        |                   ngs                  |
// +----------------------------------------+----------------------------------------+
// | # Introduction                                                                  |
// |                                                                                 |
// | <pre style="white-space: pre-wrap;white-space: -moz-pre-wrap;white-space: -pre- |
// | wrap;white-space: -o-pre-wrap;word-wrap: break-word;">                          |
// | The wave (known as the Mexican wave in the English-speaking world outside North |
// |  America) is an example of metachronal rhythm achieved in a packed stadium when |
// |  successive groups of spectators briefly stand, yell, and raise their arms. Imm |
// | ediately upon stretching to full height, the spectator returns to the usual sea |
// | ted position.                                                                   |
// |                                                                                 |
// | The result is a wave of standing spectators that travels through the crowd, eve |
// | n though individual spectators never move away from their seats. In many large  |
// | arenas the crowd is seated in a contiguous circuit all the way around the sport |
// |  field, and so the wave is able to travel continuously around the arena; in dis |
// | contiguous seating arrangements, the wave can instead reflect back and forth th |
// | rough the crowd. When the gap in seating is narrow, the wave can sometimes pass |
// |  through it. Usually only one wave crest will be present at any given time in a |
// | n arena, although simultaneous, counter-rotating waves have been produced. (Sou |
// | rce <a href="https://en.wikipedia.org/wiki/Wave_(audience)">Wikipedia</a>)      |
// | </pre>                                                                          |
// |                                                                                 |
// | # Task                                                                          |
// | <pre style="white-space: pre-wrap;white-space: -moz-pre-wrap;white-space: -pre- |
// | wrap;white-space: -o-pre-wrap;word-wrap: break-word;">                          |
// | In this simple Kata your task is to create a function that turns a string into  |
// | a Mexican Wave. You will be passed a string and you must return that string in  |
// | an array where an uppercase letter is a person standing up.                     |
// | </pre>                                                                          |
// |                                                                                 |
// | # Rules                                                                         |
// | <pre style="white-space: pre-wrap;white-space: -moz-pre-wrap;white-space: -pre- |
// | wrap;white-space: -o-pre-wrap;word-wrap: break-word;">                          |
// |  1.&nbsp; The input string will always be lower case but maybe empty.<br>       |
// |  2.&nbsp; If the character in the string is whitespace then pass over it as if  |
// | it was an empty seat                                                            |
// | </pre>                                                                          |
// |                                                                                 |                                  |
// +---------------------------------------------------------------------------------+

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
                    &sentence[..idx], // Before uppercase letter.
                    &sentence[idx..idx + 1].to_uppercase(), // The uppercase letter.
                    &sentence[idx + 1..]  // After uppercase letter.
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
