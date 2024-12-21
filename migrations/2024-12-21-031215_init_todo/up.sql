-- Your SQL goes here

CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    user_id integer NOT NULL REFERENCES users (id),
    title TEXT NOT NULL,
    is_complete BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
