-- Your SQL goes here
ALTER TABLE links ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE links ALTER COLUMN created_at SET DEFAULT NOW();
