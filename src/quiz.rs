use crate::audio::play_audio;
use crate::Word;
use colored::Colorize;
use inquire::list_option::ListOption;
use inquire::validator::{ErrorMessage, Validation};
use inquire::{MultiSelect, Select, Text};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::thread;

pub fn quiz(vocab: &Vec<Word>) {
    let mut rng = rand::thread_rng();

    let quiz_options = vec![
        ListOption::new(0, "Kanji 漢字 -> Meaning"),
        ListOption::new(1, "Kana 仮名 -> Meaning"),
        ListOption::new(2, "Kanji 漢字 -> Romaji + Meaning"),
        ListOption::new(3, "Kana 仮名 -> Romaji + Meaning"),
    ];

    let mut ask_romaji = false;
    let mut show_kana = false;
    let mut play_sound = true;

    match Select::new("Choose quiz mode:", quiz_options)
        .with_vim_mode(true)
        .prompt()
        .unwrap()
        .index
    {
        0 => {}
        1 => {
            show_kana = true;
        }
        2 => {
            ask_romaji = true;
        }
        3 => {
            show_kana = true;
            ask_romaji = true;
        }
        _ => return,
    }

    if Select::new("Want sound?", vec!["yes", "no"])
        .with_vim_mode(true)
        .prompt()
        .unwrap()
        .eq("yes")
    {
        play_sound = true;
    }

    loop {
        let random_index = rng.gen_range(0..vocab.len());
        let random_word = vocab[random_index].clone();
        let prompt = if show_kana {
            format!(
                "{} {}",
                random_word.kana.red().yellow().bold(),
                random_word.kanji.magenta().bold()
            )
        } else {
            if random_word.kanji.is_empty() {
                continue;
            }
            format!("{}", random_word.kanji.magenta().bold())
        };
        println!("{}", prompt);

        if ask_romaji {
            let validator = move |input: &str| {
                if input.eq("i want to quit") {
                    return Ok(Validation::Valid);
                }
                if input.eq("show answer") {
                    Ok(Validation::Invalid(ErrorMessage::Custom(format!(
                        "the answer is {}",
                        random_word.romaji
                    ))))
                } else if random_word.romaji.eq(input) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(ErrorMessage::Custom(
                        "Wrong Answer".into(),
                    )))
                }
            };

            let ans = Text::new("Enter romaji:".into())
                .with_validator(validator)
                .prompt()
                .unwrap();
            if ans.eq("i want to quit") {
                break;
            }
        }

        let mut other_words = vec![random_index];
        loop {
            if other_words.len() > 3 {
                break;
            }
            let x = rng.gen_range(0..vocab.len());
            if !other_words.contains(&x) {
                other_words.push(x);
            }
        }

        let mut all_meanings = vec![];
        for w in other_words {
            let mut meanings_of_this = vocab[w].meaning.clone();
            meanings_of_this.shuffle(&mut rng);
            let mut i = 0;
            for x in meanings_of_this {
                i += 1;
                all_meanings.push(x);
                if i > 2 {
                    break;
                }
            }
        }
        all_meanings.shuffle(&mut rng);
        all_meanings.push("i want to quit 😪".to_string());

        let validator = move |a: &[ListOption<&String>]| {
            if a.iter().any(|x| x.value.eq("i want to quit 😪")) {
                return Ok(Validation::Valid);
            }
            if a.is_empty() {
                return Ok(Validation::Invalid(ErrorMessage::Custom(
                    "please choose something".into(),
                )));
            }
            match a.iter().all(|x| random_word.meaning.contains(x.value)) {
                true => Ok(Validation::Valid),
                false => Ok(Validation::Invalid(ErrorMessage::Custom(
                    "incorrect answer".into(),
                ))),
            }
        };

        let ans = MultiSelect::new("Select meaning:", all_meanings)
            .with_validator(validator)
            .with_vim_mode(true)
            .prompt()
            .unwrap();

        if ans.into_iter().any(|x| x.eq("i want to quit 😪")) {
            return;
        }

        if play_sound {
            thread::spawn(|| {
                play_audio(random_word.audio);
            });
        }
    }
}