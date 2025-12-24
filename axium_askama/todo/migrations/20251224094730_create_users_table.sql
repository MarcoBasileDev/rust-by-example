-- Add migration script here
CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE IF NOT EXISTS users (
    id SERIAL NOT NULL PRIMARY KEY,
    email citext NOT NULL UNIQUE,
    password_hash bytea NOT NULL
);