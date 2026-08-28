PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS songs (
    id INTEGER PRIMARY KEY,

    title TEXT NOT NULL,
    artist TEXT NOT NULL,

    wavs TEXT NOT NULL,
    bgas TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS charts (
    id INTEGER PRIMARY KEY,

    song_id INTEGER NOT NULL,

    genre TEXT NOT NULL,
    title TEXT NOT NULL,
    subtitle TEXT,
    artist TEXT NOT NULL,
    sub_artist TEXT,

    wavs TEXT NOT NULL,
    bgas TEXT NOT NULL,

    filename TEXT NOT NULL,

    md5 TEXT UNIQUE,
    sha256 TEXT NOT NULL UNIQUE,

    updated_at INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),

    FOREIGN KEY(song_id)
        REFERENCES songs(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS tables (
    id INTEGER PRIMARY KEY,

    header_url TEXT NOT NULL,
    data_url TEXT NOT NULL,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,

    level_order TEXT NOT NULL,
    course TEXT,

    sha256 TEXT NOT NULL,
    etag TEXT,
    last_modified INTEGER,

    data BLOB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_song_title
    ON songs(title);

CREATE INDEX IF NOT EXISTS idx_song_artist
    ON songs(artist);

CREATE INDEX IF NOT EXISTS idx_chart_md5
    ON charts(md5);

CREATE INDEX IF NOT EXISTS idx_chart_sha256
    ON charts(sha256);
