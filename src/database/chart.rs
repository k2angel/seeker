use anyhow::Result;
use rusqlite::{OptionalExtension, ToSql, Transaction, params};
use serde_json::to_string;

use crate::cli::SearchExpr;
use crate::database::{Database, search_condition};
use crate::model::{Chart, ChartRow};

impl Database {
    pub fn insert_chart(conn: &Transaction<'_>, song_id: i64, chart: &Chart) -> Result<()> {
        let wavs = to_string(&chart.wavs)?;
        let bgas = to_string(&chart.bgas)?;
        let filename = chart.filename.to_string_lossy();

        let chart_id = conn
            .query_row(
                "
                SELECT id
                FROM charts
                WHERE song_id = ? AND filename = ?
                ",
                (&song_id, &filename),
                |row| row.get::<_, i64>(0),
            )
            .optional()?;

        match chart_id {
            Some(chart_id) => {
                conn.execute(
                    "
                    UPDATE charts
                    SET
                        genre = ?,
                        title = ?,
                        subtitle = ?,
                        artist = ?,
                        sub_artist = ?,
                        wavs = ?,
                        bgas = ?,
                        md5 = ?,
                        sha256 = ?
                    WHERE id = ?
                    ",
                    params![
                        &chart.genre,
                        &chart.title,
                        &chart.subtitle,
                        &chart.artist,
                        &chart.sub_artist,
                        &wavs,
                        &bgas,
                        &chart.md5,
                        &chart.sha256,
                        &chart_id,
                    ],
                )?;
            }

            None => {
                conn.execute(
                    "
                    INSERT INTO charts (
                        song_id,
                        genre,
                        title,
                        subtitle,
                        artist,
                        sub_artist,
                        wavs,
                        bgas,
                        filename,
                        md5,
                        sha256
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ",
                    params![
                        &song_id,
                        &chart.genre,
                        &chart.title,
                        &chart.subtitle,
                        &chart.artist,
                        &chart.sub_artist,
                        &wavs,
                        &bgas,
                        &chart.filename.to_string_lossy(),
                        &chart.md5,
                        &chart.sha256,
                    ],
                )?;
            }
        }

        Ok(())
    }

    pub fn search_charts(&self, expr: Option<&SearchExpr>) -> Result<Vec<ChartRow>> {
        let mut params = Vec::new();

        let sql = if let Some(expr) = &expr {
            let condition = search_condition(
                expr,
                &[
                    "genre",
                    "title",
                    "subtitle",
                    "artist",
                    "sub_artist",
                    "md5",
                    "sha256",
                ],
                &mut params,
            );

            format!(
                "
                SELECT *
                FROM charts
                WHERE {}
                ORDER BY artist, title;
                ",
                condition
            )
        } else {
            "
            SELECT *
            FROM charts
            ORDER BY artist, title;
            "
            .to_string()
        };

        let mut stmt = self.conn.prepare(&sql)?;

        let rows: Vec<ChartRow> = match expr {
            Some(_) => {
                let params: Vec<&dyn ToSql> =
                    params.iter().map(|param| param as &dyn ToSql).collect();

                let rows = stmt.query_map(params.as_slice(), ChartRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }

            None => {
                let rows = stmt.query_map([], ChartRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }
        };

        Ok(rows)
    }

    pub fn remove_chart(&self, chart_id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM charts WHERE id = ?1;", [chart_id])?;

        Ok(())
    }

    pub fn count_charts(&self, expr: Option<&SearchExpr>) -> Result<i64> {
        let mut params = Vec::new();

        let sql = if let Some(expr) = &expr {
            let condition = search_condition(
                expr,
                &[
                    "genre",
                    "title",
                    "subtitle",
                    "artist",
                    "sub_artist",
                    "md5",
                    "sha256",
                ],
                &mut params,
            );

            format!("SELECT COUNT(*) FROM charts WHERE {}", condition)
        } else {
            "SELECT COUNT(*) FROM charts".to_string()
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

    pub fn count_song_charts(&self, song_id: i64) -> Result<i64> {
        Ok(self.conn.query_row(
            "SELECT COUNT(*) FROM charts WHERE song_id = ?1",
            [song_id],
            |row| row.get(0),
        )?)
    }

    pub fn exists_chart(&self, sha256: &str) -> Result<bool> {
        Ok(self.conn.query_row(
            "
            SELECT EXISTS(
                SELECT 1
                FROM charts
                WHERE sha256 = ?1
            );
            ",
            [sha256],
            |row| row.get(0),
        )?)
    }
}
