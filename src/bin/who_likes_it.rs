// https://www.codewars.com/kata/5266876b8f4bf2da9b000362/rust

// +---------------------------------------------------------------------------------+
// |                                  Who likes it?                                  |
// +----------------------------------------+----------------------------------------+
// |                  Rank                  |                  6 kyu                 |
// +----------------------------------------+----------------------------------------+
// |                  Tags                  | Fundamentals, Formatting, Algorithms,  |
// |                                        |       Logic, Strings, Data Types       |
// +----------------------------------------+----------------------------------------+
// | You probably know the "like" system from Facebook and other pages. People can " |
// | like" blog posts, pictures or other items. We want to create the text that shou |
// | ld be displayed next to such an item.                                           |
// |                                                                                 |
// | Implement the function which takes an array containing the names of people that |
// |  like an item. It must return the display text as shown in the examples:        |
// |                                                                                 |
// | ```                                                                             |
// | []                                -->  "no one likes this"                      |
// | ["Peter"]                         -->  "Peter likes this"                       |
// | ["Jacob", "Alex"]                 -->  "Jacob and Alex like this"               |
// | ["Max", "John", "Mark"]           -->  "Max, John and Mark like this"           |
// | ["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"     |
// | ```                                                                             |
// |                                                                                 |
// |                                                                                 |
// | ```if:c                                                                         |
// | * return must be an allocated string                                            |
// | * do not mutate input                                                           |
// | ```                                                                             |
// |                                                                                 |
// | Note: For 4 or more names, the number in `"and 2 others"` simply increases.     |
// |                                                                                 |
// +---------------------------------------------------------------------------------+

fn _likes(names: &[&str]) -> String {
    let names: Vec<String> = names.into_iter().map(|name| name.to_string()).collect();
    let names_len: usize = names.len();
    let sentence: String;

    if names.is_empty() {
        return "no one likes this".to_string();
    } else {
        if names_len <= 2 {
            if names_len == 1 {
                return format!("{} likes this", names[0]);
            } else {
                sentence = names.join(" and ");
            }
        } else {
            let second_word: String = if names_len == 3 {
                names[2].clone()
            } else {
                format!("{} others", names[2..].len())
            };
            let first_word = names
                .into_iter()
                .take(2)
                .collect::<Vec<String>>()
                .join(", ");

            sentence = format!("{} and {}", first_word, second_word);
        }
    }
    format!("{} like this", sentence)
}

// OR

fn likes(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".to_string(),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, others @ ..] => format!("{}, {} and {} others like this", a, b, others.len()),
    }
}

fn main() {
    tests()
}

fn tests() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}
