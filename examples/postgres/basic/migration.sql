CREATE TABLE account (
    id             SERIAL PRIMARY KEY,
    username       TEXT UNIQUE NOT NULL
);

CREATE TABLE post (
    id             SERIAL PRIMARY KEY,
    account_id     INT NOT NULL REFERENCES account(id),
    title          TEXT NOT NULL UNIQUE,
    body           TEXT,
    published      BOOLEAN,
    details        JSONB,
    likes          INT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
