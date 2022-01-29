// https://www.codewars.com/kata/5264d2b162488dc400000001/rust

// +---------------------------------------------------------------------------------+
// |                             Stop gninnipS My sdroW!                             |
// +----------------------------------------+----------------------------------------+
// |                  Rank                  |                  6 kyu                 |
// +----------------------------------------+----------------------------------------+
// |                  Tags                  | Algorithms, Strings, Data Types, Forma |
// |                                        |              tting, Logic              |
// +----------------------------------------+----------------------------------------+
// | Write a function that takes in a string of one or more words, and returns the s |
// | ame string, but with all five or more letter words reversed (Just like the name |
// |  of this Kata). Strings passed in will consist of only letters and spaces. Spac |
// | es will be included only when more than one word is present.                    |
// |                                                                                 |
// | Examples:                                                                       |
// | spinWords( "Hey fellow warriors" ) => returns "Hey wollef sroirraw"             |
// | spinWords( "This is a test") => returns "This is a test"                        |
// | spinWords( "This is another test" )=> returns "This is rehtona test"            |
// |                                                                                 |
// +---------------------------------------------------------------------------------+

fn spin_words(words: &str) -> String {
    words
        .split(' ')
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect::<String>()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    tests()
}

fn tests() {
    assert_eq!(spin_words("Welcome"), "emocleW");
    assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(spin_words("This is a test"), "This is a test");
    assert_eq!(spin_words("This is another test"), "This is rehtona test");
    assert_eq!(
        spin_words("You are almost to the last test"),
        "You are tsomla to the last test"
    );
    assert_eq!(
        spin_words("Just kidding there is still one more"),
        "Just gniddik ereht is llits one more"
    );
    assert_eq!(
        spin_words("Seriously this is the last one"),
        "ylsuoireS this is the last one"
    );
}
