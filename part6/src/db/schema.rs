table! {
    dictionary (id) {
        id -> Integer,
        word -> Text,
        used_at -> Nullable<Date>,
        guessed -> Bool,
        language -> Text,
    }
}
