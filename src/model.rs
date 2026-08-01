use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub library: PathBuf,
    pub database: PathBuf,
}

pub struct ImportResult {
    pub song: Song,
    pub charts: Vec<Chart>,
    pub charts_due: Vec<Chart>,
}

#[derive(Debug)]
pub struct SongBase<T> {
    pub id: Option<i64>,

    pub title: String,
    pub artist: String,

    pub wavs: T,
    pub bgas: T,
}

#[derive(Debug)]
pub struct ChartBase<W, F> {
    pub id: Option<i64>,
    pub song_id: Option<i64>,

    pub genre: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub artist: String,
    pub sub_artist: Option<String>,

    pub wavs: W,
    pub bgas: W,

    pub filename: F,

    pub md5: String,
    pub sha256: String,
}

pub type Song = SongBase<Vec<PathBuf>>;
pub type SongRow = SongBase<String>;
pub type Chart = ChartBase<Vec<PathBuf>, PathBuf>;
pub type ChartRow = ChartBase<String, String>;
