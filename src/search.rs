use crate::audio::play_audio;
use crate::{ask_for_music, Word};
use colored::Colorize;
use inquire::validator::{ErrorMessage, Validation};
use inquire::Text;
use rodio::OutputStream;

pub fn romaji_search(vocab: &Vec<Word>) {
    let play_sound = ask_for_music();
    let (_stream, output_stream) = OutputStream::try_default().unwrap();

    let romaji_list = vocab.iter().map(|w| w.romaji.clone()).collect::<Vec<_>>();
    loop {
        let romaji_list_for_validator = romaji_list.clone();
        let validator = move |input: &str| {
            if input.eq("i want to quit") {
                return Ok(Validation::Valid);
            }
            if romaji_list_for_validator.contains(&input.to_string()) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(ErrorMessage::Custom(
                    "romaji not found in vocabulary".into(),
                )))
            }
        };

        let romaji_list_for_suggester = romaji_list.clone();
        let suggester = move |input: &str| {
            Ok(romaji_list_for_suggester
                .iter()
                .filter(|s| s.contains(&input.to_lowercase()))
                .map(|s| s.to_string())
                .collect())
        };

        let ans = Text::new("Enter romaji:")
            .with_validator(validator)
            .with_autocomplete(suggester)
            .prompt()
            .unwrap();
        if ans.eq("i want to quit") {
            return;
        }

        let word = vocab.iter().find(|w| w.romaji.eq(&ans)).unwrap();
        println!(
            "{} {}",
            word.kana.red().yellow().bold(),
            word.kanji.magenta()
        );

        println!("{}", "Meaning:".bright_green().bold());
        for (i, m) in word.meaning.iter().enumerate() {
            println!("{}. {}", i + 1, m.bright_green())
        }

        // clone audio to pass into the thread
        let audio = word.audio.clone();
        if play_sound {
            play_audio(audio, &output_stream);
        }
    }
}