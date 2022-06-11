use std::fs::File;
use colored::*;
use std::io::{BufRead, BufReader, stdin, Result};
use rand::seq::IteratorRandom;

/// Provides basic functions for reading and writing from and to a dictionary
pub trait Dictionary {
    fn get_random_word(&self) -> Option<DictionaryEntry>;
    fn find_word(&self, text: &str) -> Option<DictionaryEntry>;
}

/// Provides a dictionary represented by a text file
pub struct TextDictionary {
    pub dictionary_file_path: String
}

impl TextDictionary {
    /// Creates dictionary based on the file given
    ///
    /// # Arguments
    /// * `file_path` - A String representing the path to the dictionary file on the filesystem
    pub fn new(file_path: String) -> TextDictionary {
        TextDictionary { dictionary_file_path: file_path }
    }
}

pub struct DictionaryEntry {
    word: String
}

impl Dictionary for TextDictionary {
    /// Get [DictionaryEntry] from a random line of the dictionary using reservoir sampling
    fn get_random_word(&self) -> Option<DictionaryEntry> {
        let file_result = File::open(&self.dictionary_file_path);

        match file_result {
            Ok(file) => {
                let buf_reader = BufReader::new(file);

                let random_line: Option<Result<String>> = buf_reader
                    .lines()
                    .choose(&mut rand::thread_rng());

                match random_line {
                    Some(line) => Some(DictionaryEntry { word: line.unwrap() }),
                    None => None
                }
            }
            Err(e) => {
                println!("Error reading from the dictionary:\n{}", e);
                None
            }
        }
    }

    /// Search the dictionary for a specific [DictionaryEntry]
    fn find_word(&self, text: &str) -> Option<DictionaryEntry> {
        let file_result: Result<File> = File::open(&self.dictionary_file_path);

        match file_result {
            Ok(file) => {
                let buf_reader = BufReader::new(file);
                let mut word_option: Option<DictionaryEntry> = None;

                for line_result in buf_reader.lines() {
                    let line = line_result.unwrap();

                    if text.eq(line.trim()) {
                        word_option = Some(DictionaryEntry { word: String::from(line) });
                        break;
                    }
                }

                word_option
            }
            Err(error) => {
                println!("Error when looking for '{}' in the dictionary:\n{}", text, error);
                None
            }
        }
    }
}

fn main() {
    let dictionary = TextDictionary::new(String::from("res/dictionary.txt"));
    let solution_option = dictionary.get_random_word();

    match solution_option {
        None => println!("Maybe the dictionary is empty?"),
        Some(solution) => {
            let max_attempts = 6;

            let mut full_match: bool = false;

            let mut counter = 0;
            while counter < max_attempts {
                let attempt: String = read_input(5);

                match dictionary.find_word(&attempt) {
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
                    None => println!("The guessed word is not in the dictionary.")
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
        let polished = input.trim();

        if !validate_user_input(polished, word_len) {
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
