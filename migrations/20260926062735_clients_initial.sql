CREATE TABLE IF NOT EXISTS clients (
    id BLOB PRIMARY KEY NOT NULL,
    full_name TEXT NOT NULL,
    passport_number TEXT NOT NULL UNIQUE,
    address TEXT NOT NULL,
    phone TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT
);

-- Add migration script here