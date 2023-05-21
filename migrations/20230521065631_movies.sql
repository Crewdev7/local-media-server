-- Add migration script here

CREATE TABLE movies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    genre TEXT NOT NULL,
    release_year INTEGER NOT  NULL,
    created TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);


