mod audio;
mod practice;
mod quiz;
mod search;

use inquire::list_option::ListOption;
use inquire::validator::{ErrorMessage, StringValidator, Validation};
use inquire::{CustomType, CustomUserError, Select};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Clone)]
struct Word {
    #[serde(rename = "#")]
    index: usize,
    #[serde(default)]
    kanji: String,
    kana: String,
    romaji: String,
    meaning: Vec<String>,
    audio: String,
}

fn main() {
    let mut vocab: Vec<Vec<Word>> = vec![vec![]];

    for i in 1..=5 {
        let voca: Vec<Word> = serde_json::from_reader(
            File::open(format!("./vocabulary/N{}_Vocabulary_list.json", i))
                .expect("unable to open file"),
        )
        .expect("unable to deserialize");
        vocab.push(voca);
    }

    let mut level: usize = ask_for_level();

    loop {
        let main_options = vec![
            ListOption::new(0, "Search by romaji"),
            ListOption::new(1, "Practice"),
            ListOption::new(2, "Quiz"),
            ListOption::new(3, "Change JLPT level"),
            ListOption::new(4, "Exit"),
        ];
        match Select::new("What you want to do?", main_options)
            .with_vim_mode(true)
            .prompt()
            .unwrap()
            .index
        {
            0 => search::romaji_search(&vocab[level]),
            1 => practice::practice(&vocab[level]),
            2 => quiz::quiz(&vocab[level]),
            3 => {
                level = ask_for_level();
            }
            _ => break,
        }
    }
}

fn ask_for_level() -> usize {
    CustomType::new("JLPT Level:")
        .with_formatter(&|i: usize| format!("{i}"))
        .with_validator(|i: &usize| {
            if *i > 5 {
                Ok(Validation::Invalid(ErrorMessage::Default))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_error_message("Please type a valid level")
        .prompt()
        .unwrap()
}

#[derive(Clone)]
struct AskForRomaji {
    romaji: String,
}

impl AskForRomaji {
    fn new(romaji: String) -> Self {
        Self { romaji }
    }
}

impl StringValidator for AskForRomaji {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if input.eq("i want to quit") {
            return Ok(Validation::Valid);
        }
        if input.eq("show answer") {
            Ok(Validation::Invalid(ErrorMessage::Custom(format!(
                "the answer is {}",
                self.romaji
            ))))
        } else if self.romaji.eq(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(ErrorMessage::Custom(
                "Wrong Answer".into(),
            )))
        }
    }
}

fn ask_for_music() -> bool {
    if Select::new("Want sound?", vec!["yes", "no"])
        .with_vim_mode(true)
        .prompt()
        .unwrap()
        .eq("yes")
    {
        true
    } else {
        false
    }
}