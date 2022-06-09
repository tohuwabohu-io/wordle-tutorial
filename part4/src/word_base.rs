/// Provides basic functions for reading and writing from and to a word base
pub trait WordBase {
    fn get_random_word(&self) -> Option<WordEntry>;
    fn find_word(&self, text: &str) -> Option<WordEntry>;
    fn create_word(&self, word_entry: WordEntry);
}

/// Represents a word base entry
pub struct WordEntry {
    pub word: String
}