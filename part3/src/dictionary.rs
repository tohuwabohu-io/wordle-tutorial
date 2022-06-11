/// Provides basic functions for reading and writing from and to a dictionary
pub trait Dictionary {
    fn get_random_word(&self) -> Option<DictionaryEntry>;
    fn find_word(&self, text: &str) -> Option<DictionaryEntry>;
}

/// Represents a dictionary entry
pub struct DictionaryEntry {
    pub(crate) word: String
}