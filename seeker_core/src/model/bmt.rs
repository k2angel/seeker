use anyhow::Result;
use flate2::Compression;
use flate2::write::GzEncoder;
use serde::Serialize;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::utils;

#[derive(Debug, Serialize)]
pub enum Class {
    #[serde(rename = "bms.player.beatoraja.TableData$TableFolder")]
    TableFolder,

    #[serde(rename = "bms.player.beatoraja.song.SongData")]
    SongData,

    #[serde(rename = "bms.player.beatoraja.CourseData")]
    CourseData,

    #[serde(rename = "bms.player.beatoraja.CourseData$TrophyData")]
    TrophyData,
}

#[derive(Debug, Serialize)]
pub enum Constraint {
    #[serde(rename = "CLASS")]
    Class,
    #[serde(rename = "MIRROR")]
    Mirror,
    #[serde(rename = "RANDOM")]
    Random,
    #[serde(rename = "NO_SPEED")]
    NoSpeed,
    #[serde(rename = "NO_GOOD")]
    NoGood,
    #[serde(rename = "NO_GREAT")]
    NoGreat,
    #[serde(rename = "GAUGE_LR2")]
    GaugeLr2,
    #[serde(rename = "GAUGE_5KEYS")]
    Gauge5Keys,
    #[serde(rename = "GAUGE_7KEYS")]
    Gauge7Keys,
    #[serde(rename = "GAUGE_9KEYS")]
    Gauge9Keys,
    #[serde(rename = "GAUGE_24KEYS")]
    Gauge24Keys,
    #[serde(rename = "LN")]
    Ln,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "HCN")]
    Hcn,
}

#[derive(Debug, Serialize)]
pub struct Bmt {
    pub url: String,
    pub name: String,
    pub tag: String,
    pub folder: Vec<Folder>,
    pub course: Option<Vec<Course>>,
}

#[derive(Debug, Serialize)]
pub struct Folder {
    pub class: Class,
    pub name: String,
    pub songs: Vec<Song>,
}

#[derive(Debug, Serialize)]
pub struct Song {
    pub class: Class,
    pub title: String,
    pub artist: Option<String>,
    pub md5: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Course {
    pub class: Class,
    pub name: String,
    pub hash: Vec<Hash>,
    pub constraint: Vec<Constraint>,
    pub trophy: Vec<Trophy>,
}

#[derive(Debug, Serialize)]
pub struct Hash {
    pub title: String,
    pub md5: String,
}

#[derive(Debug, Serialize)]
pub struct Trophy {
    pub class: Class,
    pub name: String,
    pub missrate: f32,
    pub scorerate: f32,
}

impl TryFrom<&str> for Constraint {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "grade" => Ok(Self::Class),
            "grade_mirror" => Ok(Self::Mirror),
            "grade_random" => Ok(Self::Random),
            "no_speed" => Ok(Self::NoSpeed),
            "no_good" => Ok(Self::NoGood),
            "no_great" => Ok(Self::NoGreat),
            "gauge_lr2" => Ok(Self::GaugeLr2),
            "gauge_5k" => Ok(Self::Gauge5Keys),
            "gauge_7k" => Ok(Self::Gauge7Keys),
            "gauge_9k" => Ok(Self::Gauge9Keys),
            "gauge_24k" => Ok(Self::Gauge24Keys),
            "ln" => Ok(Self::Ln),
            "cn" => Ok(Self::Cn),
            "hcn" => Ok(Self::Hcn),
            _ => Err(()),
        }
    }
}

impl Bmt {
    fn get_path(&self, dir: &Path) -> Result<PathBuf> {
        Ok(dir.join(format!(
            "table/{}.bmt",
            utils::sha256sum(self.url.as_bytes())
        )))
    }

    pub fn write(&self, dir: &Path) -> Result<()> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&serde_json::to_vec(&self)?)?;

        let bmt = encoder.finish()?;
        std::fs::write(self.get_path(dir)?, bmt)?;

        let config_path = dir.join("config_sys.json");
        let mut config: serde_json::Value =
            serde_json::from_reader(std::fs::File::open(&config_path)?)?;

        let table_url = config.get_mut("tableURL").and_then(|v| v.as_array_mut());

        match table_url {
            Some(urls) => {
                if !urls.iter().any(|url| url.as_str() == Some(&self.url)) {
                    urls.push(serde_json::Value::String(self.url.clone()));
                }
            }
            None => {
                config["tableURL"] = serde_json::json!([self.url]);
            }
        }

        std::fs::write(config_path, serde_json::to_vec_pretty(&config)?)?;

        Ok(())
    }

    pub fn remove(&self, dir: &Path) -> Result<()> {
        std::fs::remove_file(self.get_path(dir)?)?;

        Ok(())
    }
}
