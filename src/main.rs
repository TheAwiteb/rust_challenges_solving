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
            .expect(&format!(
                "Could not read {}",
                self.file.as_path().to_str().unwrap()
            ))
            .split("\n")
            .collect::<Vec<&str>>()[0]
            .to_string()
            .strip_prefix("// ")
            .unwrap()
            .to_string()
    }

    fn filename(&self) -> String {
        self.file
            .as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn name(&self) -> String {
        self.filename()
            .replace("_", " ")
            .strip_suffix(".rs")
            .unwrap()
            .to_string()
    }
}

fn get_challenges(challenges_dir: &Path) -> Vec<Challenge> {
    fs::read_dir(challenges_dir)
        .expect("Unable to open challenges_dir")
        .map(|entry| {
            let entry = entry.expect("unable to get entry");
            let file_name = entry.file_name();
            if file_name != std::ffi::OsString::from("main.rs") {
                Some(file_name.to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .filter(|file| file.is_some())
        .map(|file| Challenge::new(challenges_dir.join(file.unwrap())))
        .collect()
}

fn main() {
    let challenges_dir = Path::new("./src");
    let challenges: Vec<Challenge> = get_challenges(challenges_dir);

    println!("Total challenges: {}", challenges.len());

    for (challenge_num, challenge) in challenges.iter().enumerate() {
        println!(
            "\n🛡🛡🛡🛡🛡🛡🛡  {} 🛡🛡🛡🛡🛡🛡🛡\n\nChallenge file 📁: {}\nChallenge name 🎯: {}\nCodeWars url   🔗: {}",
            challenge_num + 1,
            challenge.file.as_os_str().to_str().unwrap(),
            challenge.name(),
            challenge.url()
        );
    }
}
