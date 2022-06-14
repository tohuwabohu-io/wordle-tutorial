create table dictionary (
    id integer primary key not null,
    word varchar not null,
    used_at DATE null,
    guessed boolean default 0 not null,
    language varchar not null
)
