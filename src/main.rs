extern crate rand;

include!("letter/letter.rs");
include!("game/enum.rs");

// using rand library
use rand::{Rng, thread_rng};

// importing file and io libraries
use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    println!("Welcome to Hangman!");

    loop {
        println!("\nYou have {} turns left.", turns_left);
        display_progress(&letters);


        println!("\nPlease enter a letter to guess:");
        let user_char = read_user_input_character();

        if user_char == '*' {
            break;
        }

        let mut at_least_on_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_on_revealed = true;
            }
        }

        if !at_least_on_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats, you won! The word was {}", selected_word);
                break;
            }
            GameProgress::Lost => {
                println!("\nSorry, you lost!");
                break;
            }
        }
    }

    println!("\nGoodbye!");
}

fn select_word() -> String {
    /* Open file */
    let mut file = File::open("words.txt").expect("couldn't open file!");

    /* Load file contents */
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("an error occurred while reading file!");

    /* Get individual words */
    let available_words: Vec<&str> = file_contents.trim().split(",").collect();

    /* Generating random index */
    let random_index = thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    /* Wrap each character in a letter struct */
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    /* Display appropriate character */
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    return match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { c }
                None => { '*' }
            }
        }
        Err(_) => { '*' }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;

            break;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
