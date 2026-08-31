use anyhow::Result;
use rusqlite::{ToSql, Transaction, params, params_from_iter};
use serde_json::{from_str, to_string};
use std::collections::HashSet;
use std::path::PathBuf;

use crate::database::{Database, search_condition};
use crate::model::{SearchExpr, Song, SongRow};
use crate::utils;

impl Database {
    pub fn insert_song(conn: &Transaction<'_>, song: &Song) -> Result<i64> {
        let wavs = to_string(&song.wavs)?;
        let bgas = to_string(&song.bgas)?;

        conn.execute(
            "
            INSERT INTO songs (
                title,
                artist,
                wavs,
                bgas
            ) VALUES (?, ?, ?, ?)
            ",
            params![&song.title, &song.artist, &wavs, &bgas],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn update_song_resources(conn: &Transaction<'_>, song_id: i64) -> Result<()> {
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

            for wav in from_str::<Vec<PathBuf>>(&chart_wavs)? {
                wavs.insert(wav);
            }

            for bga in from_str::<Vec<PathBuf>>(&chart_bgas)? {
                bgas.insert(bga);
            }
        }

        let wavs = to_string(&wavs.into_iter().collect::<Vec<_>>())?;
        let bgas = to_string(&bgas.into_iter().collect::<Vec<_>>())?;

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

    pub fn get_song(&self, song_id: i64) -> Result<SongRow> {
        let mut stmt = self.conn.prepare(
            "
            SELECT *
            FROM songs
            WHERE id = ?1;
            ",
        )?;

        Ok(stmt.query_row([song_id], SongRow::from_row)?)
    }

    pub fn get_songs<I>(&self, song_ids: I) -> Result<Vec<SongRow>>
    where
        I: IntoIterator<Item = i64>,
    {
        let song_ids: Vec<i64> = song_ids.into_iter().collect();

        let placeholders = std::iter::repeat_n("?", song_ids.len())
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

        let rows = stmt.query_map(params_from_iter(song_ids.iter()), SongRow::from_row)?;

        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn find_song(&self, song: &Song) -> Result<Option<SongRow>> {
        let rows: Vec<SongRow> = {
            let title = format!("{}%", utils::tokenize(&song.title)[0].value);
            let artist = format!("{}%", utils::tokenize(&song.artist)[0].value);

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

        Ok(match rows.len() {
            0 => None,
            _ => {
                let mut best: Option<(SongRow, f64)> = None;

                for row in rows {
                    let wavs = from_str::<Vec<PathBuf>>(&row.wavs)?;
                    let matches = song.wavs.iter().filter(|wav| wavs.contains(wav)).count();
                    let rate = matches as f64 / song.wavs.len() as f64;

                    if rate >= 0.5
                        && best
                            .as_ref()
                            .map(|(_, best_rate)| rate > *best_rate)
                            .unwrap_or(true)
                    {
                        best = Some((row, rate));
                    }
                }

                best.map(|(row, _)| row)
            }
        })
    }

    pub fn search_songs(&self, expr: Option<&SearchExpr>) -> Result<Vec<SongRow>> {
        let mut params = Vec::new();

        let sql = if let Some(expr) = &expr {
            let condition = search_condition(expr, &["title", "artist"], &mut params);

            format!(
                "
                SELECT *
                FROM songs
                WHERE {}
                ORDER BY artist, title;
                ",
                condition
            )
        } else {
            "
            SELECT *
            FROM songs
            ORDER BY artist, title;
            "
            .to_string()
        };

        let mut stmt = self.conn.prepare(&sql)?;

        Ok(match expr {
            Some(_) => {
                let params: Vec<&dyn ToSql> =
                    params.iter().map(|param| param as &dyn ToSql).collect();

                let rows = stmt.query_map(params.as_slice(), SongRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }

            None => {
                let rows = stmt.query_map([], SongRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }
        })
    }

    pub fn remove_song(&self, song_id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM songs WHERE id = ?1;", [song_id])?;

        Ok(())
    }

    pub fn count_songs(&self, expr: Option<&SearchExpr>) -> Result<i64> {
        let mut params = Vec::new();

        let sql = if let Some(expr) = &expr {
            let condition = search_condition(expr, &["title", "artist"], &mut params);

            format!("SELECT COUNT(*) FROM songs WHERE {}", condition)
        } else {
            "SELECT COUNT(*) FROM songs".to_string()
        };

        let mut stmt = self.conn.prepare(&sql)?;

        Ok(match expr {
            Some(_) => {
                let params: Vec<&dyn ToSql> =
                    params.iter().map(|param| param as &dyn ToSql).collect();

                stmt.query_row(params.as_slice(), |row| row.get(0))?
            }

            None => stmt.query_row([], |row| row.get(0))?,
        })
    }
}
