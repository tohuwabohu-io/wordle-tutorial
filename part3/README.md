# fancy-hangman-part3
part 3 of the [fancy-hangman tutorial](https://www.tohuwabohu.io/2022/06/building-a-cli-wordle-game-in-rust-part-3/)

fancy-hangman-rs is a wordle inspired word guessing game for the CLI written in rust.

## Game rules
The player has to correctly guess a randomly selected word from the word base. All words are 5 characters long. By coloring single letters the game tells the player about correct letter positioning.
* green: The guessed letter is at the correct position.
* orange: The word contains the letter, but at a different position.

The game ends when the player runs out of guesses or when the player guesses the word correctly. After that, a message is displayed. 