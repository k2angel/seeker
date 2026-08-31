use anyhow::Result;
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

pub trait HasMetadata {
    fn title(&self) -> &str;
    fn artist(&self) -> &str;

    fn directory_name(&self) -> String {
        let name = format!("[{}] {}", self.artist(), self.title());

        name.chars()
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

    fn library_dir(&self, library: &Path) -> PathBuf {
        library.join(self.directory_name())
    }
}

impl HasMetadata for Song {
    fn title(&self) -> &str {
        &self.title
    }

    fn artist(&self) -> &str {
        &self.artist
    }
}

impl HasMetadata for SongRow {
    fn title(&self) -> &str {
        &self.title
    }

    fn artist(&self) -> &str {
        &self.artist
    }
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
