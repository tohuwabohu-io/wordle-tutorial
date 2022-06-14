table! {
    dictionary (id) {
        id -> Int4,
        word -> Varchar,
        used_at -> Nullable<Date>,
        guessed -> Bool,
    }
}
