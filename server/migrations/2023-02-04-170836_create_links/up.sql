-- Your SQL goes here
CREATE TABLE links (
    id SERIAL PRIMARY KEY,
    shared_link TEXT NOT NULL,
    original_link TEXT NOT NULL UNIQUE
)
