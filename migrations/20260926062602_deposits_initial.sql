CREATE TABLE IF NOT EXISTS deposits (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    term_months INTEGER NOT NULL,
    rate_percent REAL NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT
);

-- Add migration script here