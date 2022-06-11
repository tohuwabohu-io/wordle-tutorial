use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, LineWriter, Result, Write};
use rand::seq::IteratorRandom;
use crate::dictionary::{Dictionary, DictionaryEntry};

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

    fn create_word(&self, word_entry: DictionaryEntry) {
        match self.find_word(&word_entry.word) {
            Some(_) => println!("'{}' already exists in the dictionary.", &word_entry.word),
            None => {
                let file_result = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.dictionary_file_path);

                match file_result {
                    Ok(file) => {
                        let mut writer: LineWriter<File> = LineWriter::new(file);
                        writer.write(&word_entry.word.as_ref()).unwrap();
                        writer.write(b"\n").unwrap();

                        println!("Added '{}' to the dictionary!", &word_entry.word)
                    }
                    Err(e) => println!("Error when writing '{}' to the dictionary:\n{}", &word_entry.word, e)
                };
            }
        };
    }
}