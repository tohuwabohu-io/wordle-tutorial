use colored::*;
use std::io::stdin;

fn main() {
    let solution: String = String::from("gusty").to_lowercase();
    let max_attempts = 6;

    let mut full_match: bool = false;

    let mut counter = 0;
    while counter < max_attempts {
        let attempt: String = read_input(5);

        let guesses = max_attempts - counter - 1;

        full_match = check_word(&solution, &attempt);

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
    }

    if full_match == true {
        println!("Congratulations! You won!");
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