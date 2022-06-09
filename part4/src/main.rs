use colored::*;
use std::io::stdin;

use fancy_hangman::lang::locale::{get_app_language, replace_unicode};
use fancy_hangman::text::text_word_base::TextWordBase;
use fancy_hangman::word_base::WordBase;

fn main() {
    let word_base = TextWordBase::new(String::from("res/word_base.txt"));
    let solution_option = word_base.get_random_word();

    match solution_option {
        None => println!("Maybe the word base is empty?"),
        Some(solution) => {
            let max_attempts = 6;

            let mut full_match: bool = false;

            let mut counter = 0;
            while counter < max_attempts {
                let attempt: String = read_input(5);

                match word_base.find_word(&attempt) {
                    Some(_) => {
                        let guesses: i32 = max_attempts - counter - 1;
                        full_match = check_word(&solution.word, &attempt);

                        if full_match == true {
                            break;
                        } else {
                            if guesses > 1 {
                                println!("You now have {} guesses.", guesses);
                            } else {
                                println!("This is your last guess.");
                            }
                        }

                        if guesses == 0 { println!("Better luck next time!") }

                        counter += 1;
                    },
                    None => println!("The guessed word is not in the word list.")
                }
            }

            if full_match == true {
                println!("Congratulations! You won!");
            }
        }
    }
}

fn read_input(word_len: usize) -> String {
    let mut input: String = String::new();

    loop {
        stdin().read_line(&mut input).unwrap();
        let polished = replace_unicode(input.trim(), get_app_language());

        if !validate_user_input(&polished, word_len) {
            println!("Invalid input: Your guess must have a size of {} characters. You entered {} characters.", word_len, polished.len());

            input.clear();
        } else {
            input = polished.to_lowercase();

            break;
        }
    }

    input
}

fn validate_user_input(user_input: &str, expected_len: usize) -> bool {
    user_input.len() == expected_len
}

fn check_word(solution_word: &str, guessed_word: &str) -> bool {
    let guessed_characters: Vec<char> = guessed_word.chars().collect();
    let solution_characters: Vec<char> = solution_word.chars().collect();

    for i in 0..guessed_word.len() {
        let index: Option<usize> = solution_word.find(guessed_characters[i]);

        match index {
            Some(_index) => {
                if solution_characters[i] == guessed_characters[i] {
                    print!("{} ", guessed_characters[i].to_string().color("green"))
                } else {
                    print!("{} ", guessed_characters[i].to_string().color("yellow"))
                }
            }
            None => { print!("{} ", guessed_characters[i]) }
        }
    }

    println!();

    // check for full match
    if String::from(solution_word).eq(guessed_word) {
        return true;
    }

    false
}

#[cfg(test)]
use fancy_hangman::lang::locale::AppLanguage;

#[cfg(test)]
#[test]
fn test_validate_user_input() {
    assert!(validate_user_input(
        replace_unicode("schön", AppLanguage::DE).as_str(), 6
    ));

    assert!(validate_user_input(
        replace_unicode("schön", AppLanguage::EN).as_str(), 5
    ));

    assert!(validate_user_input(
        replace_unicode("lüge", AppLanguage::DE).as_str(), 5
    ));

    assert!(validate_user_input(
        replace_unicode("lüge", AppLanguage::EN).as_str(), 4
    ));

    assert!(validate_user_input(
        replace_unicode("howdy", AppLanguage::DE).as_str(), 5
    ));

    assert!(validate_user_input(
        replace_unicode("howdy", AppLanguage::EN).as_str(), 5
    ));
}