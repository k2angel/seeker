use anyhow::Result;
use flate2::Compression;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, from_str};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::model::bmt;
use crate::utils;

pub struct Table {
    pub header_url: String,
    pub data_url: String,
    pub name: String,
    pub symbol: String,

    pub level_order: Vec<String>,
    pub course: Option<Vec<Vec<Course>>>,

    pub sha256: String,
    pub etag: Option<String>,
    pub last_modified: Option<i64>,

    pub data: Vec<u8>,
}

pub struct TableRow {
    pub id: i64,

    pub header_url: String,
    pub data_url: String,
    pub name: String,
    pub symbol: String,

    pub level_order: String,
    pub course: Option<String>,

    pub sha256: String,
    pub etag: Option<String>,
    pub last_modified: Option<i64>,

    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Header {
    pub data_url: String,
    pub name: String,
    pub symbol: String,

    #[serde(alias = "levels")]
    pub level_order: Option<Vec<String>>,
    pub course: Option<Vec<Vec<Course>>>,
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub md5: Option<String>,
    pub sha256: Option<String>,
    pub level: String,

    pub title: String,
    pub artist: String,

    pub url: Option<String>,
    pub url_diff: Option<String>,
    pub name_diff: Option<String>,
    pub url_pack: Option<String>,
    pub name_pack: Option<String>,
    pub comment: Option<String>,
    pub org_md5: Option<String>,
    pub mode: Option<String>,
    pub ipfs: Option<String>,
    pub ipfs_diff: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    pub name: String,
    pub constraint: Vec<String>,
    pub trophy: Vec<Trophy>,
    pub charts: Option<Vec<CourseData>>,

    pub md5: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trophy {
    pub name: String,
    pub missrate: f32,
    pub scorerate: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseData {
    pub md5: String,
    pub title: String,
}

impl Table {
    fn to_bmt(&self) -> Result<Vec<u8>> {
        let data: Vec<Data> = from_slice(&self.data)?;
        let mut folders: HashMap<String, Vec<bmt::Song>> = HashMap::new();

        for row in &data {
            let song = bmt::Song {
                class: bmt::Class::SongData,
                title: row.title.clone(),
                artist: row.artist.clone(),
                md5: row.md5.clone(),
                sha256: row.sha256.clone(),
            };

            let name = format!("{}{}", self.symbol, row.level);

            folders.entry(name).or_default().push(song);
        }

        let mut folder = folders
            .into_iter()
            .map(|(name, songs)| bmt::Folder {
                class: bmt::Class::TableFolder,
                name,
                songs,
            })
            .collect::<Vec<_>>();

        folder.sort_by_key(|folder| {
            folder
                .name
                .strip_prefix(&self.symbol)
                .and_then(|level| self.level_order.iter().position(|x| x == level))
                .unwrap_or(usize::MAX)
        });

        let course = self.course.as_ref().map(|course| {
            course
                .iter()
                .flat_map(|courses| {
                    courses.iter().map(|course| {
                        let hash = course
                            .charts
                            .as_ref()
                            .map(|charts| {
                                charts
                                    .iter()
                                    .map(|chart| bmt::Hash {
                                        title: chart.title.clone(),
                                        md5: chart.md5.clone(),
                                    })
                                    .collect()
                            })
                            .or_else(|| {
                                course.md5.as_ref().map(|md5s| {
                                    md5s.iter()
                                        .enumerate()
                                        .map(|(i, md5)| bmt::Hash {
                                            title: format!("course {}", i + 1),
                                            md5: md5.clone(),
                                        })
                                        .collect()
                                })
                            })
                            .unwrap_or_default();

                        bmt::Course {
                            class: bmt::Class::CourseData,
                            name: course.name.clone(),
                            hash,
                            constraint: course
                                .constraint
                                .clone()
                                .iter()
                                .filter_map(|c| bmt::Constraint::try_from(c.as_str()).ok())
                                .collect(),
                            trophy: course
                                .trophy
                                .iter()
                                .map(|trophy| bmt::Trophy {
                                    class: bmt::Class::TrophyData,
                                    name: trophy.name.clone(),
                                    missrate: trophy.missrate,
                                    scorerate: trophy.scorerate,
                                })
                                .collect(),
                        }
                    })
                })
                .collect()
        });

        let bmt = bmt::Bmt {
            url: self.header_url.clone(),
            name: self.name.clone(),
            tag: self.symbol.clone(),
            folder,
            course,
        };

        Ok(serde_json::to_vec(&bmt)?)
    }

    fn get_bmt(&self, dir: &Path) -> Result<PathBuf> {
        Ok(dir.join(format!(
            "table/{}.bmt",
            utils::sha256sum(self.header_url.as_bytes())
        )))
    }

    pub fn write_bmt(&self, dir: &Path) -> Result<()> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&self.to_bmt()?)?;

        let bmt = encoder.finish()?;
        std::fs::write(self.get_bmt(dir)?, bmt)?;

        let config_path = dir.join("config_sys.json");
        let mut config: serde_json::Value =
            serde_json::from_reader(std::fs::File::open(&config_path)?)?;

        let table_url = config.get_mut("tableURL").and_then(|v| v.as_array_mut());

        match table_url {
            Some(urls) => {
                if !urls
                    .iter()
                    .any(|url| url.as_str() == Some(&self.header_url))
                {
                    urls.push(serde_json::Value::String(self.header_url.clone()));
                }
            }
            None => {
                config["tableURL"] = serde_json::json!([self.header_url]);
            }
        }

        std::fs::write(config_path, serde_json::to_vec_pretty(&config)?)?;

        Ok(())
    }

    pub fn remove_bmt(&self, dir: &Path) -> Result<()> {
        std::fs::remove_file(self.get_bmt(dir)?)?;

        Ok(())
    }
}

impl TableRow {
    pub fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            header_url: row.get("header_url")?,
            data_url: row.get("data_url")?,
            name: row.get("name")?,
            symbol: row.get("symbol")?,
            level_order: row.get("level_order")?,
            course: row.get("course")?,
            sha256: row.get("sha256")?,
            etag: row.get("etag")?,
            last_modified: row.get("last_modified")?,
            data: row.get("data")?,
        })
    }
}

impl TryFrom<TableRow> for Table {
    type Error = anyhow::Error;

    fn try_from(row: TableRow) -> Result<Self> {
        let course = row
            .course
            .as_deref()
            .map(serde_json::from_str)
            .transpose()?;

        Ok(Self {
            header_url: row.header_url,
            data_url: row.data_url,
            name: row.name,
            symbol: row.symbol,
            level_order: from_str(&row.level_order)?,
            course,
            sha256: row.sha256,
            etag: row.etag,
            last_modified: row.last_modified,
            data: row.data,
        })
    }
}
