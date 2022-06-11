use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, LineWriter, Result, Write};
use rand::seq::IteratorRandom;
use crate::word_base::{WordBase, WordEntry};

/// Provides a word base represented by a text file
pub struct TextWordBase {
    pub wordbase_file_path: String
}

impl TextWordBase {
    /// Creates word base based on the file given
    ///
    /// # Arguments
    /// * `file_path` - A String representing the path to the word base file on the filesystem
    pub fn new(file_path: String) -> TextWordBase {
        TextWordBase { wordbase_file_path: file_path }
    }
}

impl WordBase for TextWordBase {
    /// Get [WordEntry] from a random line of the wordbase using reservoir sampling
    fn get_random_word(&self) -> Option<WordEntry> {
        let file_result = File::open(&self.wordbase_file_path);

        match file_result {
            Ok(file) => {
                let buf_reader = BufReader::new(file);

                let random_line: Option<Result<String>> = buf_reader
                    .lines()
                    .choose(&mut rand::thread_rng());

                match random_line {
                    Some(line) => Some(WordEntry { word: line.unwrap() }),
                    None => None
                }
            }
            Err(e) => {
                println!("Error reading from the wordbase:\n{}", e);
                None
            }
        }
    }

    /// Search the wordbase for a specific [WordEntry]
    fn find_word(&self, text: &str) -> Option<WordEntry> {
        let file_result: Result<File> = File::open(&self.wordbase_file_path);

        match file_result {
            Ok(file) => {
                let buf_reader = BufReader::new(file);
                let mut word_option: Option<WordEntry> = None;

                for line_result in buf_reader.lines() {
                    let line = line_result.unwrap();

                    if text.eq(line.trim()) {
                        word_option = Some(WordEntry { word: String::from(line) });
                        break;
                    }
                }

                word_option
            }
            Err(error) => {
                println!("Error when looking for '{}' in the wordbase:\n{}", text, error);
                None
            }
        }
    }

    fn create_word(&self, word_entry: WordEntry) {
        match self.find_word(&word_entry.word) {
            Some(_) => println!("'{}' already exists in the wordbase.", &word_entry.word),
            None => {
                let file_result = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.wordbase_file_path);

                match file_result {
                    Ok(file) => {
                        let mut writer: LineWriter<File> = LineWriter::new(file);
                        writer.write(&word_entry.word.as_ref()).unwrap();
                        writer.write(b"\n").unwrap();

                        println!("Added '{}' to the wordbase!", &word_entry.word)
                    }
                    Err(e) => println!("Error when writing '{}' to the wordbase:\n{}", &word_entry.word, e)
                };
            }
        };
    }
}