// https://www.codewars.com/kata/559536379512a64472000053/rust/

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
