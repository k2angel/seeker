use anyhow::Result;
use rusqlite;
use serde_json::from_str;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Chart {
    pub genre: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub artist: String,
    pub sub_artist: Option<String>,

    pub wavs: Vec<PathBuf>,
    pub bgas: Vec<PathBuf>,

    pub filename: PathBuf,

    pub md5: Option<String>,
    pub sha256: String,
}

pub struct ChartRow {
    pub id: i64,
    pub song_id: i64,

    pub genre: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub artist: String,
    pub sub_artist: Option<String>,

    pub wavs: String,
    pub bgas: String,

    pub filename: String,

    pub md5: Option<String>,
    pub sha256: String,
}

impl ChartRow {
    pub fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            song_id: row.get("song_id")?,
            genre: row.get("genre")?,
            title: row.get("title")?,
            subtitle: row.get("subtitle")?,
            artist: row.get("artist")?,
            sub_artist: row.get("sub_artist")?,
            wavs: row.get("wavs")?,
            bgas: row.get("bgas")?,
            filename: row.get("filename")?,
            md5: row.get("md5")?,
            sha256: row.get("sha256")?,
        })
    }
}

impl TryFrom<ChartRow> for Chart {
    type Error = anyhow::Error;

    fn try_from(row: ChartRow) -> Result<Self> {
        Ok(Self {
            genre: row.genre,
            title: row.title,
            subtitle: row.subtitle,
            artist: row.artist,
            sub_artist: row.sub_artist,
            wavs: from_str(&row.wavs)?,
            bgas: from_str(&row.bgas)?,
            filename: PathBuf::from(row.filename),
            md5: row.md5,
            sha256: row.sha256,
        })
    }
}
