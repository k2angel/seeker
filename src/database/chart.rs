use anyhow::Result;
use std::path::PathBuf;

use crate::database::Database;
use crate::model::{Chart, ChartRow};

impl ChartRow {
    pub fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            song_id: row.get(1)?,
            genre: row.get(2)?,
            title: row.get(3)?,
            subtitle: row.get(4)?,
            artist: row.get(5)?,
            sub_artist: row.get(6)?,
            wavs: row.get(7)?,
            bgas: row.get(8)?,
            filename: row.get(9)?,
            md5: row.get(10)?,
            sha256: row.get(11)?,
        })
    }
}

impl TryFrom<ChartRow> for Chart {
    type Error = anyhow::Error;

    fn try_from(row: ChartRow) -> Result<Self> {
        Ok(Self {
            id: row.id,
            song_id: row.song_id,
            genre: row.genre,
            title: row.title,
            subtitle: row.subtitle,
            artist: row.artist,
            sub_artist: row.sub_artist,
            wavs: serde_json::from_str(&row.wavs)?,
            bgas: serde_json::from_str(&row.bgas)?,
            filename: PathBuf::from(row.filename),
            md5: row.md5,
            sha256: row.sha256,
        })
    }
}

impl Database {
    pub fn insert_chart(
        conn: &rusqlite::Transaction<'_>,
        song_id: i64,
        chart: &Chart,
    ) -> Result<()> {
        let wavs = serde_json::to_string(&chart.wavs)?;
        let bgas = serde_json::to_string(&chart.bgas)?;

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
            (
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
            ),
        )?;

        Ok(())
    }

    pub fn search_charts(&self, query: Option<String>) -> Result<Vec<Chart>> {
        let sql = if query.is_some() {
            "
            SELECT *
            FROM charts
            WHERE
                title      LIKE ?1 OR
                subtitle   LIKE ?1 OR
                artist     LIKE ?1 OR
                sub_artist LIKE ?1
            ORDER BY
                title;
            "
        } else {
            "
            SELECT *
            FROM charts
            ORDER BY
                title;
            "
        };

        let mut stmt = self.conn.prepare(sql)?;

        let rows: Vec<ChartRow> = match query {
            Some(query) => {
                let pattern = format!("%{}%", query);

                let rows = stmt.query_map([&pattern], ChartRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }

            None => {
                let rows = stmt.query_map([], ChartRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }
        };

        let charts = rows
            .into_iter()
            .map(Chart::try_from)
            .collect::<Result<Vec<_>>>()?;

        Ok(charts)
    }

    pub fn remove_chart(&self, chart_id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM charts WHERE id = ?1;", [chart_id])?;

        Ok(())
    }

    pub fn count_charts(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM charts", [], |row| row.get(0))?)
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
