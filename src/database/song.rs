use anyhow::Result;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::cli::SearchExpr;
use crate::database::{Database, search_condition};
use crate::model::{Song, SongRow};
use crate::utils;

impl SongRow {
    pub fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            wavs: row.get(3)?,
            bgas: row.get(4)?,
        })
    }
}

impl TryFrom<SongRow> for Song {
    type Error = anyhow::Error;

    fn try_from(row: SongRow) -> Result<Self> {
        Ok(Self {
            id: row.id,
            title: row.title,
            artist: row.artist,
            wavs: serde_json::from_str(&row.wavs)?,
            bgas: serde_json::from_str(&row.bgas)?,
        })
    }
}

impl Database {
    pub fn insert_song(conn: &rusqlite::Transaction<'_>, song: &Song) -> Result<i64> {
        let wavs = serde_json::to_string(&song.wavs)?;
        let bgas = serde_json::to_string(&song.bgas)?;

        conn.execute(
            "
            INSERT INTO songs (
                title,
                artist,
                wavs,
                bgas
            ) VALUES (?, ?, ?, ?)
            ",
            (&song.title, &song.artist, &wavs, &bgas),
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn rebuild_song_resources(conn: &rusqlite::Transaction<'_>, song_id: i64) -> Result<()> {
        let mut stmt = conn.prepare(
            "
            SELECT wavs, bgas
            FROM charts
            WHERE song_id = ?1
            ",
        )?;

        let mut wavs = HashSet::new();
        let mut bgas = HashSet::new();

        let rows = stmt.query_map([song_id], |row| {
            let wavs: String = row.get(0)?;
            let bgas: String = row.get(1)?;

            Ok((wavs, bgas))
        })?;

        for row in rows {
            let (chart_wavs, chart_bgas) = row?;

            for wav in serde_json::from_str::<Vec<PathBuf>>(&chart_wavs)? {
                wavs.insert(wav);
            }

            for bga in serde_json::from_str::<Vec<PathBuf>>(&chart_bgas)? {
                bgas.insert(bga);
            }
        }

        let wavs = serde_json::to_string(&wavs.into_iter().collect::<Vec<_>>())?;
        let bgas = serde_json::to_string(&bgas.into_iter().collect::<Vec<_>>())?;

        conn.execute(
            "
            UPDATE songs
            SET wavs = ?1,
                bgas = ?2
            WHERE id = ?3
            ",
            (&wavs, &bgas, &song_id),
        )?;

        Ok(())
    }

    pub fn detail_song(&self, song_id: i64) -> Result<Song> {
        let mut stmt = self.conn.prepare(
            "
            SELECT *
            FROM songs
            WHERE id = ?1;
            ",
        )?;

        let row = stmt.query_row([song_id], SongRow::from_row)?;

        Ok(Song::try_from(row)?)
    }

    pub fn detail_songs<I>(&self, song_ids: I) -> Result<HashMap<i64, Song>>
    where
        I: IntoIterator<Item = i64>,
    {
        let song_ids: Vec<i64> = song_ids.into_iter().collect();

        if song_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = std::iter::repeat("?")
            .take(song_ids.len())
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "
            SELECT *
            FROM songs
            WHERE id IN ({});
            ",
            placeholders
        );

        let mut stmt = self.conn.prepare(&sql)?;

        let rows = stmt.query_map(
            rusqlite::params_from_iter(song_ids.iter()),
            SongRow::from_row,
        )?;

        let songs = rows
            .map(|row| {
                let song = Song::try_from(row?)?;
                Ok((song.id.unwrap(), song))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(songs)
    }

    pub fn find_songs(&self, song: &Song) -> Result<Vec<Song>> {
        let rows: Vec<SongRow> = {
            let title = format!("{}%", utils::tokenize(&song.title)[0].0);
            let artist = format!("{}%", utils::tokenize(&song.artist)[0].0);

            let mut stmt = self.conn.prepare(
                "
                SELECT *
                FROM songs
                WHERE
                    title LIKE ?1 AND
                    artist LIKE ?2
                ORDER BY title, artist;
                ",
            )?;

            let rows = stmt.query_map([title, artist], SongRow::from_row)?;
            rows.collect::<rusqlite::Result<_>>()?
        };

        let songs = rows
            .into_iter()
            .map(Song::try_from)
            .collect::<Result<Vec<_>>>()?;

        match songs.len() {
            0 => Ok(Vec::new()),
            1 => Ok(songs),
            _ => todo!(),
        }
    }

    pub fn search_songs(&self, expr: Option<&SearchExpr>) -> Result<Vec<Song>> {
        let mut params = Vec::new();

        let sql = if expr.is_some() {
            let condition =
                search_condition(expr.as_ref().unwrap(), &["title", "artist"], &mut params);

            format!(
                "
                SELECT *
                FROM songs
                WHERE {}
                ORDER BY title;
                ",
                condition
            )
        } else {
            "
            SELECT *
            FROM songs
            ORDER BY title;
            "
            .to_string()
        };

        let mut stmt = self.conn.prepare(&sql)?;

        let rows: Vec<SongRow> = match expr {
            Some(_) => {
                let params: Vec<&dyn rusqlite::ToSql> = params
                    .iter()
                    .map(|param| param as &dyn rusqlite::ToSql)
                    .collect();

                let rows = stmt.query_map(params.as_slice(), SongRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }

            None => {
                let rows = stmt.query_map([], SongRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }
        };

        let songs = rows
            .into_iter()
            .map(Song::try_from)
            .collect::<Result<Vec<_>>>()?;

        Ok(songs)
    }

    pub fn remove_song(&self, song_id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM songs WHERE id = ?1;", [song_id])?;

        Ok(())
    }

    pub fn count_songs(&self, expr: Option<&SearchExpr>) -> Result<i64> {
        let mut params = Vec::new();

        let sql = if expr.is_some() {
            let condition =
                search_condition(expr.as_ref().unwrap(), &["title", "artist"], &mut params);

            format!("SELECT COUNT(*) FROM songs WHERE {}", condition)
        } else {
            "SELECT COUNT(*) FROM songs".to_string()
        };

        let mut stmt = self.conn.prepare(&sql)?;

        Ok(match expr {
            Some(_) => {
                let params: Vec<&dyn rusqlite::ToSql> = params
                    .iter()
                    .map(|param| param as &dyn rusqlite::ToSql)
                    .collect();

                stmt.query_row(params.as_slice(), |row| row.get(0))?
            }

            None => stmt.query_row([], |row| row.get(0))?,
        })
    }
}
