CREATE TABLE IF NOT EXISTS accounts (
    id BLOB PRIMARY KEY NOT NULL,
    client_id BLOB NOT NULL,
    deposit_id BLOB NOT NULL,
    opened_at TEXT NOT NULL,
    closed_at TEXT,
    amount REAL NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT,
    CONSTRAINT clients_fk FOREIGN KEY (client_id) REFERENCES clients (id),
    CONSTRAINT deposits_fk FOREIGN KEY (deposit_id) REFERENCES deposits (id)
);

-- Add migration script here