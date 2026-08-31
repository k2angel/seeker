use anyhow::Result;
use rusqlite::{OptionalExtension, params};
use serde_json::to_string;

use crate::database::Database;
use crate::model::table::{Table, TableRow};

impl Database {
    pub fn insert_table(&self, table: &Table) -> Result<()> {
        let level_order = to_string(&table.level_order)?;
        let course = if let Some(course) = &table.course {
            Some(to_string(course)?)
        } else {
            None
        };

        self.conn.execute(
            "
            INSERT INTO tables (
                header_url,
                data_url,
                name,
                symbol,
                level_order,
                course,
                sha256,
                etag,
                last_modified,
                data
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ",
            params![
                table.header_url,
                table.data_url,
                table.name,
                table.symbol,
                level_order,
                course,
                table.sha256,
                table.etag,
                table.last_modified,
                table.data,
            ],
        )?;

        Ok(())
    }

    pub fn update_table(&self, table_id: i64, table: &Table) -> Result<()> {
        let level_order = to_string(&table.level_order)?;
        let course = if let Some(course) = &table.course {
            Some(to_string(course)?)
        } else {
            None
        };

        self.conn.execute(
            "
            UPDATE tables
            SET
                header_url = ?,
                data_url = ?,
                name = ?,
                symbol = ?,
                level_order = ?,
                course = ?,
                sha256 = ?,
                etag = ?,
                last_modified = ?,
                data = ?
            WHERE id = ?
            ",
            params![
                table.header_url,
                table.data_url,
                table.name,
                table.symbol,
                level_order,
                course,
                table.sha256,
                table.etag,
                table.last_modified,
                table.data,
                table_id,
            ],
        )?;

        Ok(())
    }

    pub fn remove_table(&self, table_id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM tables WHERE id = ?", params![table_id])?;

        Ok(())
    }

    pub fn get_table(&self, name: &str) -> Result<Option<TableRow>> {
        let mut stmt = self.conn.prepare("SELECT * FROM tables WHERE name = ?1;")?;

        let row: Option<TableRow> = stmt.query_row([name], TableRow::from_row).optional()?;

        Ok(row)
    }

    pub fn get_tables(&self, query: Option<String>) -> Result<Vec<TableRow>> {
        let sql = if query.is_some() {
            "
            SELECT *
            FROM tables
            WHERE name LIKE ?1
                OR symbol LIKE ?1
                OR EXISTS (
                    SELECT 1
                    FROM json_each(level_order)
                    WHERE symbol || value = ?2
                );
            "
        } else {
            "SELECT * FROM tables ORDER BY header_url, name;"
        };

        let mut stmt = self.conn.prepare(sql)?;

        let rows: Vec<TableRow> = match query {
            Some(query) => {
                let pattern = format!("%{}%", query);
                let rows = stmt.query_map([&pattern, &query], TableRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }

            None => {
                let rows = stmt.query_map([], TableRow::from_row)?;
                rows.collect::<rusqlite::Result<_>>()?
            }
        };

        Ok(rows)
    }
}
