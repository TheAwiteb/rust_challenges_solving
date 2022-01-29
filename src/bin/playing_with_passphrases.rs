// https://www.codewars.com/kata/559536379512a64472000053/rust/

// +---------------------------------------------------------------------------------+
// |                             Playing with passphrases                            |
// +----------------------------------------+----------------------------------------+
// |                  Rank                  |                  6 kyu                 |
// +----------------------------------------+----------------------------------------+
// |                  Tags                  | Algorithms, Chars, Data Types, Strings |
// |                                        |            , Encoding, Logic           |
// +----------------------------------------+----------------------------------------+
// | Everyone knows passphrases. One can choose passphrases from poems, songs, movie |
// | s names and so on but frequently                                                |
// | they can be guessed due to common cultural references.                          |
// | You  can get your passphrases stronger by different means. One is the following |
// | :                                                                               |
// |                                                                                 |
// | choose a text in capital letters including or not digits and non alphabetic cha |
// | racters,                                                                        |
// |                                                                                 |
// | 1. shift each letter by a given number but the transformed letter must be a let |
// | ter (circular shift),                                                           |
// | 2. replace each digit by its complement to 9,                                   |
// | 3. keep such as non alphabetic and non digit characters,                        |
// | 4. downcase each letter in odd position, upcase each letter in even position (t |
// | he first character is in position 0),                                           |
// | 5. reverse the whole result.                                                    |
// |                                                                                 |
// | #### Example:                                                                   |
// |                                                                                 |
// | your text: "BORN IN 2015!", shift 1                                             |
// |                                                                                 |
// | 1 + 2 + 3 -> "CPSO JO 7984!"                                                    |
// |                                                                                 |
// | 4 "CpSo jO 7984!"                                                               |
// |                                                                                 |
// | 5 "!4897 Oj oSpC"                                                               |
// |                                                                                 |
// | With longer passphrases it's better to have a small and easy program.           |
// | Would you write it?                                                             |
// |                                                                                 |
// | https://en.wikipedia.org/wiki/Passphrase                                        |
// +---------------------------------------------------------------------------------+

#[allow(dead_code)]
fn play_pass(text: &str, shift: u32) -> String {
    let mut chars_arr = text
        .chars()
        .map(|chr| {
            if chr.is_digit(10) {
                format!("{}", 9 - chr.to_digit(10).unwrap())
            } else if chr.is_alphabetic() {
                let mut chr_val = (chr as u8) + shift as u8;
                if (91..=96).contains(&chr_val) {
                    chr_val = (96 + (chr_val - 90)) as u8
                } else if (chr_val > 96) && ((chr as u8) < 91) {
                    chr_val += 6
                }
                format!("{}", chr_val as char)
            } else {
                format!("{}", chr)
            }
        })
        .enumerate()
        .map(|(idx, chr)| {
            if idx % 2 == 0 {
                chr.to_uppercase()
            } else {
                chr.to_lowercase()
            }
        })
        .collect::<Vec<String>>();
    chars_arr.reverse();
    chars_arr.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, n: u32, exp: &str) -> () {
        println!(" s: {:?};", s);
        println!("n: {:?};", n);
        let ans = play_pass(s, n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("I LOVE YOU!!!", 1, "!!!vPz fWpM J");
        dotest("I LOVE YOU!!!", 0, "!!!uOy eVoL I");
        dotest("AAABBCCY", 1, "zDdCcBbB");
    }
}

fn main() {}
