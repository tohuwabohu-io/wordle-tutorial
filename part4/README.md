# fancy-hangman-part4
part 4 of the [fancy-hangman tutorial](https://www.tohuwabohu.io/2022/06/building-a-cli-wordle-game-in-rust-part-4/)

fancy-hangman-rs is a wordle inspired word guessing game for the CLI written in rust.

## Game rules
The player has to correctly guess a randomly selected word from the dictionary. All words are 5 characters long. By coloring single letters the game tells the player about correct letter positioning.
* green: The guessed letter is at the correct position.
* orange: The word contains the letter, but at a different position.

The game ends when the player runs out of guesses or when the player guesses the word correctly. After that, a message is displayed.

## Usage

Run the game by executing `cargo run` or `cargo run --bin game`

Run the importer tool by executing `cargo run --bin import <source_path> [language]`

If `language` is not set, it defaults to `"en"`.

## Import
The import tool can be used to expand the dictionary. The tool automatically removes duplicates and entries with a size different of 5 characters and converts unicode characters to ASCII using any_ascii. German umlauts receive a special treatment.
