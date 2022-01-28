// https://www.codewars.com/kata/55c45be3b2079eccff00010f/rust

#[allow(dead_code)]
fn order(sentence: &str) -> String {
    let mut words: Vec<&str> = sentence.split_whitespace().collect();
    words.sort_by_key(|word| word.chars().find(|c| c.is_numeric()).unwrap_or('0'));
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
// your_order,_please
fn main() {}
