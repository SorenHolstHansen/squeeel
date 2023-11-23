CREATE TABLE users (
    id             SERIAL PRIMARY KEY,
    username       TEXT UNIQUE NOT NULL
);

CREATE TABLE posts (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL REFERENCES users(id),
    title          TEXT NOT NULL UNIQUE,
    body           TEXT,
    published      BOOLEAN,
    likes          INT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);