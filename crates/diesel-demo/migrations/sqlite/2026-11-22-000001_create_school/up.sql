CREATE TABLE school (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    lon REAL NOT NULL,
    lat REAL NOT NULL,
    created_at TIMESTAMP NOT NULL
);
