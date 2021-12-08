/*
Print all existing challenges, their path, name and their url in CodeWars.com
*/
use std::fs;
use std::path::{Path, PathBuf};

struct Challenge {
    file: PathBuf,
}

impl Challenge {
    fn new(file: PathBuf) -> Challenge {
        Challenge { file }
    }
}

impl Challenge {
    fn url(&self) -> String {
        fs::read_to_string(&self.file)
            // use `.display()` method that would return valid unicode string
            // and replace invalid text with ? mark
            .expect(&format!("Could not read {}", self.file.display()))
            .lines()
            // get 1st line
            .next()
            // error if there were no lines
            .expect("Make sure you provide a URL at 1st line in the file")
            .strip_prefix("// ")
            // error if the URL line is empty
            .expect("URL shouldn't be empty, make sure you write URL after `// `")
            .to_string()
    }

    fn filename(&self) -> String {
        self.file
            .file_name()
            // error if filename end with `..` which is unlikely
            .expect("expect to find a valid file name")
            // remove none unicode chars
            .to_string_lossy()
            .to_string()
    }

    fn name(&self) -> String {
        self.filename()
            .strip_suffix(".rs")
            .expect("challenge filename must end with `.rs`")
            .replace("_", " ")
    }
}

trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for str {
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        if let Some(first_char) = chars.next() {
            // if text is not empty
            first_char.to_uppercase().to_string() + chars.as_str()
        } else {
            // if text is empty, return self (which is empty) as string
            self.to_string()
        }
    }
}

fn get_challenges(challenges_dir: impl AsRef<Path>) -> Vec<Challenge> {
    fs::read_dir(challenges_dir)
        .expect("Unable to open challenges_dir")
        // - we should return `PathBuf`, not String so we don't need to use
        //   unwrap on filename
        // - we can use `filter_map` to do `map` + `filter` in same time
        .filter_map(|entry| {
            let entry = entry.expect("unable to get entry");
            if entry.file_name() != "main.rs" {
                Some(Challenge::new(entry.path()))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let challenges: Vec<Challenge> = get_challenges("./src/bin/");

    println!("Total challenges: {}", challenges.len());

    for (challenge_num, challenge) in challenges.iter().enumerate() {
        println!(
            "\
            🛡🛡🛡🛡🛡🛡🛡 {} 🛡🛡🛡🛡🛡🛡🛡\n\n\

            Challenge file 📁: {}\n\
            Challenge name 🎯: {}\n\
            CodeWars url   🔗: {}\n\
            ",
            challenge_num + 1,
            challenge.file.display(),
            challenge.name().capitalize(),
            challenge.url()
        );
    }
}
