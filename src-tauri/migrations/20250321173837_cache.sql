-- Add migration script here

CREATE TABLE IF NOT EXISTS image_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    path TEXT NOT NULL,
    last_modified INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_image_cache_url ON image_cache (url);
