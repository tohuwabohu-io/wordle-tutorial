use crate::db::db_dictionary::DbDictionary;
use crate::lang::locale::AppLanguage;
use crate::text::text_dictionary::TextDictionary;

use std::env;
use dotenv::dotenv;

/// Provides basic functions for reading and writing from and to a dictionary
pub trait Dictionary {
    fn get_random_word(&self) -> Option<DictionaryEntry>;
    fn find_word(&self, text: &str) -> Option<DictionaryEntry>;
    fn create_word(&self, word_entry: DictionaryEntry) -> Option<DictionaryEntry>;
}

/// Represents a dictionary entry
pub struct  DictionaryEntry {
    pub word: String
}

pub fn get_dictionary(app_language: AppLanguage, dictionary_flag: String) -> Box<dyn Dictionary> {
    match dictionary_flag.as_str() {
        "db" =>  {
            dotenv().ok();
            Box::new(DbDictionary::new(
                env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                app_language
            ))
        },
        _ => Box::new(TextDictionary::new(format!("res/dictionary_{}.txt", app_language.to_string()))),
    }
}
