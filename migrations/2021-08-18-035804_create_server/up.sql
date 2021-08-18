-- Your SQL goes here

CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    guild BIGINT NOT NULL,
    joined TIMESTAMPTZ NOT NULL,
    blacklisted BOOLEAN NOT NULL DEFAULT 'f'
)