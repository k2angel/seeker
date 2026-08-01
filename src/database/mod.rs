mod chart;
mod song;

use anyhow::Result;
use std::path::Path;

// const VERSION: i32 = 1;
const SCHEMA: &str = include_str!("schema.sql");

pub struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = rusqlite::Connection::open(path)?;

        Ok(Self { conn })
    }

    pub fn create_schema(&self) -> Result<()> {
        self.conn.execute_batch(SCHEMA)?;

        Ok(())
    }

    pub fn transaction(&mut self) -> Result<rusqlite::Transaction<'_>> {
        Ok(self.conn.transaction()?)
    }
}
