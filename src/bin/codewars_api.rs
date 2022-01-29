// Not Found!

// This mod will create a table with the challenge information and write it in the challenge file
// Don't forget to add CodeWars url at the beginning of the file 💥
// Using:
// cargo run --bin codewars_api <file>

use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};
use url::Url;

const CODEWARS_REGEX: &str = r"https://(www.|)codewars.com/kata/(.+)(/|$)";
const CODEWARS_API: &str = "https://www.codewars.com/api/v1/code-challenges/";

#[derive(Serialize, Deserialize, Debug)]
struct Rank {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Challenge {
    description: String,
    name: String,
    rank: Rank,
    tags: Vec<String>,
}

/// Returns if the url is valid Codewars url
/// # Examples
/// ```rust
/// assert_eq!(is_valid_challenge_url(&Url::parse("https://www.codewars.com/some_path").unwrap()),
///     false);
///
/// assert_eq!(is_valid_challenge_url(&Url::parse("https://google.com/some_path").unwrap()),
///     false);
///
/// assert_eq!(is_valid_challenge_url(&Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f/").unwrap()),
///     true);
///
/// assert_eq!(is_valid_challenge_url(&Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f").unwrap()),
///     true);
/// ```
///
fn is_valid_challenge_url(url: &Url) -> bool {
    let re = Regex::new(CODEWARS_REGEX).unwrap();
    re.is_match(&url.to_string())
}

/// Returns Codewars challenge id if is valid url else returns None
/// # Examples
/// ```rust
/// assert_eq!(extern_challenge_id(&Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f/rust").unwrap()),
///     Some(String::from("55c45be3b2079eccff00010f")));
///
/// assert_eq!(extern_challenge_id(&Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f/").unwrap()),
///     Some(String::from("55c45be3b2079eccff00010f")));
///
/// assert_eq!(extern_challenge_id(&Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f").unwrap()),
///     Some(String::from("55c45be3b2079eccff00010f")));
/// ```
fn extern_challenge_id(url: &Url) -> Option<String> {
    if is_valid_challenge_url(url) {
        let re = Regex::new(CODEWARS_REGEX).unwrap();
        Some(
            re.captures(&url.to_string())
                .unwrap()
                .get(2)
                .unwrap() // Unwrap Because the above link has been verified
                .as_str()
                .split("/")
                .take(1)
                .collect(),
        )
    } else {
        None
    }
}

/// Returns Codewars challenge api Url
/// # Examples
/// ```rust
/// assert_eq!(
///     get_api_url(
///         &Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f/").unwrap()
///     ),
///     Url::parse("https://www.codewars.com/api/v1/code-challenges/55c45be3b2079eccff00010f")
///         .unwrap()
/// );
///
/// assert_eq!(
///     get_api_url(
///         &Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f/rust").unwrap()
///     ),
///     Url::parse("https://www.codewars.com/api/v1/code-challenges/55c45be3b2079eccff00010f")
///         .unwrap()
/// );
///
/// assert_eq!(
///     get_api_url(&Url::parse("https://www.codewars.com/kata/55c45be3b2079eccff00010f").unwrap()),
///     Url::parse("https://www.codewars.com/api/v1/code-challenges/55c45be3b2079eccff00010f")
///         .unwrap()
/// );
/// ```
///
fn get_api_url(url: &Url) -> Url {
    Url::parse(&format!(
        "{}{}",
        CODEWARS_API,
        match extern_challenge_id(&url) {
            Some(url) => url,
            _ => panic!("Invalid Codewars URL: {}", url),
        }
    ))
    .unwrap() // Unwrap Because we are sure that the link is correct
}

impl From<Url> for Challenge {
    fn from(url: Url) -> Challenge {
        reqwest::blocking::get(get_api_url(&url))
            .expect("There was an error while sending request")
            .json()
            .expect("Cannot get text of response")
    }
}

impl From<&str> for Challenge {
    fn from(url: &str) -> Challenge {
        Challenge::from(Url::parse(url).expect("Invalid url"))
    }
}

fn get_url_from_file(file: &PathBuf) -> String {
    fs::read_to_string(&file)
        // use `.display()` method that would return valid unicode string
        // and replace invalid text with ? mark
        .expect(&format!("Could not read {}", file.display()))
        .lines()
        // get 1st line
        .next()
        // error if there were no lines
        .expect("Make sure you provide a URL at 1st line in the file")
        .strip_prefix("// ")
        // error if the URL line is empty
        .expect("URL shouldn't be empty, make sure you write URL after `// `")
        .to_owned()
}

fn get_file_description(file: &PathBuf) -> String {
    let challenge: Challenge = Challenge::from(get_url_from_file(&file).as_ref());

    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::simple();

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        challenge.name,
        2,
        Alignment::Center,
    )]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Rank", 1, Alignment::Center),
        TableCell::new_with_alignment(challenge.rank.name, 1, Alignment::Center),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Tags", 1, Alignment::Center),
        TableCell::new_with_alignment(challenge.tags.join(", "), 1, Alignment::Center),
    ]));

    table.add_row(Row::new(vec![TableCell::new_with_col_span(
        format!("{}", challenge.description),
        2,
    )]));

    table
        .render()
        .lines()
        .map(|line| format!("// {}", line))
        .collect::<Vec<String>>()
        .join("\n")
}

fn write_description_in_file(file: &PathBuf) {
    let file_content = fs::read_to_string(file).expect("Cannot Read File");
    let description_line = "// challenge description here";
    if file_content.contains(description_line) {
        fs::write(
            &file,
            file_content.replace(description_line, get_file_description(&file).as_ref()),
        )
        .expect("Cannot write in file");
    } else {
        panic!(
            "Add `{}` to file to replace it with a description",
            description_line
        )
    }
}

fn main() {
    let file_path: String = std::env::args().nth(1).expect("No file path specified");
    write_description_in_file(&PathBuf::from(&file_path));
}
