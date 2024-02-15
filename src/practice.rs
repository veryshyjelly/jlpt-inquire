use crate::audio::play_audio;
use crate::{ask_for_music, AskForRomaji, Word};
use colored::Colorize;
use inquire::list_option::ListOption;
use inquire::{Select, Text};
use rand::Rng;
use rodio::OutputStream;

pub fn practice(vocab: &Vec<Word>) {
    let mut rng = rand::thread_rng();

    let practice_options = vec![
        ListOption::new(0, "Kanji 漢字 -> Romaji"),
        ListOption::new(1, "Kana 仮名 -> Romaji"),
        ListOption::new(2, "Kanji 漢字 -> Romaji with meaning"),
        ListOption::new(3, "Kana 仮名 -> Romaji with meaning"),
    ];

    let mut show_meaning = false;
    let mut show_kana = false;

    match Select::new("What you want to practice?", practice_options)
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
            show_meaning = true;
        }
        3 => {
            show_kana = true;
            show_meaning = true;
        }
        _ => return,
    }
    let play_sound = ask_for_music();
    let (_stream, output_stream) = OutputStream::try_default().unwrap();

    loop {
        let random_index = rng.gen_range(0..vocab.len());
        let random_word = vocab[random_index].clone();
        let prompt = if show_kana {
            format!(
                "{} {}",
                random_word.kana.red().yellow().bold(),
                random_word.kanji.magenta()
            )
        } else {
            if random_word.kanji.is_empty() {
                continue;
            }
            format!("{}", random_word.kanji.magenta())
        };

        let ans = Text::new(&prompt)
            .with_validator(AskForRomaji::new(random_word.romaji))
            .prompt()
            .unwrap();
        if ans.eq("i want to quit") {
            break;
        }
        if show_meaning {
            println!("{}", "Meaning:".bright_green().bold());
            for (i, m) in random_word.meaning.into_iter().enumerate() {
                println!("{}. {}", i + 1, m.bright_green())
            }
        }
        if play_sound {
            play_audio(random_word.audio, &output_stream);
        }
    }
}