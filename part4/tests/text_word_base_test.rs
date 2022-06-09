use fancy_hangman::text::text_word_base::TextWordBase;
use fancy_hangman::word_base::{WordBase, WordEntry};

use crate::tools::{fill, get_sample_words, setup, teardown};


#[test]
fn test_create_word() {
    let file_path = setup();

    let word_base = TextWordBase::new(file_path.clone());

    word_base.create_word(WordEntry{ word: String::from("rusty") });

    teardown(file_path);
}

#[test]
fn test_find_word() {
    let file_path = setup();
    fill(&file_path, get_sample_words());

    let word_base = TextWordBase::new(file_path.clone());

    for word_str in get_sample_words() {
        match word_base.find_word(word_str) {
            Some(word) => assert_eq!(word_str, word.word),
            None => assert!(false)
        }
    }

    teardown(file_path);
}

#[test]
fn test_find_word_negative() {
    let file_path = setup();

    let word_base = TextWordBase::new(file_path.clone());

    for word_str in get_sample_words() {
        match word_base.find_word(word_str) {
            Some(_) => assert!(false),
            None => assert!(true)
        }
    }

    teardown(file_path);
}

#[test]
fn test_read_random_word() {
    let file_path = setup();

    fill(&file_path, get_sample_words());

    let word_base = TextWordBase::new(file_path.clone());

    match word_base.get_random_word() {
        Some(word) =>
            assert!(get_sample_words().contains(&word.word.as_str())),
        None => assert!(false)
    }

    teardown(file_path);
}

#[test]
fn test_read_random_word_negative() {
    let file_path = setup();

    let word_base = TextWordBase::new(file_path.clone());

    match word_base.get_random_word() {
        Some(_) => assert!(false),
        None => assert!(true)
    }

    teardown(file_path);
}

mod tools {
    use std::env::temp_dir;
    use std::fs::{File, OpenOptions, remove_file};
    use std::io::Write;
    use uuid::Uuid;

    pub fn setup() -> String {
        let tmp_file_name = format!("{}/{}.txt", temp_dir().to_str().unwrap(), Uuid::new_v4());

        File::create(&tmp_file_name).unwrap();

        tmp_file_name
    }

    pub fn fill(file_path: &String, sample_words: Vec<&str>) {
        let file_result =  OpenOptions::new()
            .append(true)
            .open(file_path);

        match file_result {
            Ok(mut file) => {
                for word in sample_words {
                    file.write(word.as_ref()).unwrap();
                    file.write(b"\n").unwrap();
                }
            }
            Err(e) => panic!("Error setting up integration test:\n{}", e)
        };
    }

    pub fn get_sample_words() -> Vec<&'static str> {
        vec!["rusty", "fishy", "busty", "lusty"]
    }

    pub fn teardown(file_path: String) {
        remove_file(file_path).unwrap();
    }
}