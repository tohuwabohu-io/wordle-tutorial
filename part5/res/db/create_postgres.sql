create table dictionary (
    id serial primary key,
    word varchar not null,
    used_at DATE null,
    guessed boolean default 'f' not null,
    language varchar not null
)
