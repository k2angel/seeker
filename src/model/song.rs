use anyhow::Result;
use rusqlite;
use serde_json::from_str;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Song {
    pub title: String,
    pub artist: String,

    pub wavs: Vec<PathBuf>,
    pub bgas: Vec<PathBuf>,
}

pub struct SongRow {
    pub id: i64,

    pub title: String,
    pub artist: String,

    pub wavs: String,
    pub bgas: String,
}

impl SongRow {
    pub fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            title: row.get("title")?,
            artist: row.get("artist")?,
            wavs: row.get("wavs")?,
            bgas: row.get("bgas")?,
        })
    }
}

impl TryFrom<SongRow> for Song {
    type Error = anyhow::Error;

    fn try_from(row: SongRow) -> Result<Self> {
        Ok(Self {
            title: row.title,
            artist: row.artist,
            wavs: from_str(&row.wavs)?,
            bgas: from_str(&row.bgas)?,
        })
    }
}

impl Song {
    pub fn directory_name(&self) -> String {
        fn sanitize_filename(s: &str) -> String {
            s.chars()
                .map(|c| match c {
                    '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
                    c if c.is_control() => '_',
                    _ => c,
                })
                .collect::<String>()
                .trim()
                .trim_end_matches('.')
                .to_string()
        }

        sanitize_filename(&format!("[{}] {}", self.artist, self.title))
    }

    pub fn library_dir(&self, library: &Path) -> PathBuf {
        library.join(self.directory_name())
    }
}
