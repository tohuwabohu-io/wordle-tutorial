use chrono::NaiveDate;

use super::schema::dictionary;

#[derive(Queryable)]
pub struct DbDictionaryEntry {
    pub id: i32,
    pub word: String,
    pub used_at: Option<NaiveDate>,
    pub guessed: bool,
    pub language: String
}

#[derive(Insertable)]
#[table_name="dictionary"]
pub struct NewDbDictionaryEntry {
    pub word: String,
    pub language: String
}