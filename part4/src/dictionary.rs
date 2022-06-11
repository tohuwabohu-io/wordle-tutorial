/// Provides basic functions for reading and writing from and to a dictionary
pub trait Dictionary {
    fn get_random_word(&self) -> Option<DictionaryEntry>;
    fn find_word(&self, text: &str) -> Option<DictionaryEntry>;
    fn create_word(&self, word_entry: DictionaryEntry);
}

/// Represents a dictionary entry
pub struct DictionaryEntry {
    pub word: String
}