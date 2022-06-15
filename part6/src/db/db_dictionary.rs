use chrono::NaiveDate;
use diesel::dsl::sql;
use diesel::{Connection, ExpressionMethods, OptionalExtension, SqliteConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;

use crate::db::model::{DbDictionaryEntry, NewDbDictionaryEntry};
use crate::db::schema::dictionary;
use crate::dictionary::{Dictionary, DictionaryEntry};
use crate::lang::locale::AppLanguage;

pub struct DbDictionary {
    conn: SqliteConnection,
    app_language: AppLanguage
}

impl DbDictionary {
    pub fn new(db_url: String, app_language: AppLanguage) -> DbDictionary {
        DbDictionary {
            conn: SqliteConnection::establish(&db_url)
                .expect(&format!("Error connecting to database {}", db_url)),
            app_language
        }
    }

    fn update_entry(&self, entry: &DbDictionaryEntry) {
        match diesel::update(dictionary::dsl::dictionary
            .filter(dictionary::id.eq(entry.id)))
            .set((dictionary::used_at.eq(entry.used_at), dictionary::guessed.eq(entry.guessed)))
            .execute(&self.conn) {
                Ok(affected_rows) => if affected_rows <= 0 { println!("No rows were affected when updating {}", entry.id) },
                Err(error) => println!("Error when updating entry with id {}:\n{}", entry.id, error)
        }
    }

    fn get_word_of_today(&self, current_day: NaiveDate) -> Result<Option<DbDictionaryEntry>, Error> {
        match dictionary::dsl::dictionary
            .filter(dictionary::used_at.eq(current_day))
            .filter(dictionary::language.eq(&self.app_language.to_string()))
            .limit(1)
            .get_result::<DbDictionaryEntry>(&self.conn)
            .optional() {
                Err(error) => Err(error),
                Ok(result) => match result {
                    Some(entry) => Ok(Some(entry)),
                    None => dictionary::dsl::dictionary
                        .filter(dictionary::used_at.is_null())
                        .filter(dictionary::language.eq(&self.app_language.to_string()))
                        .order(sql::<()>("RANDOM()"))
                        .limit(1)
                        .get_result::<DbDictionaryEntry>(&self.conn)
                        .optional()
            }
        }
    }
}

impl Dictionary for DbDictionary {
    /// Return a randomly selected word that has not been used before.
    /// If found, the database entry will be updated with a [chrono::NaiveDate] matching today.
    fn get_random_word(&self) -> Option<DictionaryEntry> {
        let current_day = chrono::Utc::now();
        let current_day: NaiveDate = current_day.naive_utc().date();

        match self.get_word_of_today(current_day) {
            Ok(result) => match result {
                None => None,
                Some(mut entry) => {
                    entry.used_at = Some(current_day);

                    self.update_entry(&entry);
                    Some(DictionaryEntry{
                        word: entry.word,
                        guessed: entry.guessed
                    })
                }
            }
            Err(error) => {
                println!("Error when getting today's word.\n{}", error);

                None
            }
        }
    }

    fn find_word(&self, text: &str) -> Option<DictionaryEntry> {
        let db_result = dictionary::dsl::dictionary
            .filter(dictionary::word.eq(text))
            .filter(dictionary::language.eq(&self.app_language.to_string()))
            .get_result::<DbDictionaryEntry>(&self.conn)
            .optional();

        match db_result {
            Ok(db_word) => match db_word {
                Some(entry) => Some(DictionaryEntry {
                    word: entry.word,
                    guessed: entry.guessed
                }),
                None => None
            },
            Err(error) => {
                println!("Error when looking for '{}' in the database:\n{}", text, error);

                None
            }
        }
    }

    fn create_word(&self, word_entry: DictionaryEntry) -> Option<DictionaryEntry> {
        match self.find_word(&word_entry.word) {
            None => {
                let new_word = NewDbDictionaryEntry {
                    word: String::from(&word_entry.word),
                    language: self.app_language.to_string()
                };

                let db_result = diesel::insert_into(dictionary::table)
                    .values(&new_word)
                    .execute(&self.conn);

                match db_result {
                    Ok(_) => {
                        Some(word_entry)
                    },
                    Err(e) => {
                        println!("Error when writing '{}' to the database:\n{}", &new_word.word, e);
                        None
                    }
                }
            },
            Some (_) => None
        }
    }

    fn guessed_word(&self, word_entry: DictionaryEntry) {
        match diesel::update(dictionary::dsl::dictionary
            .filter(dictionary::word.eq(word_entry.word)))
            .filter(dictionary::language.eq(&self.app_language.to_string()))
            .set(dictionary::guessed.eq(true))
            .execute(&self.conn) {
                Ok(_) => {},
                Err(error) => { println!("Error updating the solution.\n{}", error) }
        }
    }
}