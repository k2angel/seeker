use anyhow::Result;
use chrono::{DateTime, Local};

use crate::database::Database;
use crate::model::Config;
use crate::table::load::load_table;

pub fn run(config: &Config, urls: Vec<String>) -> Result<()> {
    let mut db = Database::open(&config.library)?;
    db.create_schema()?;

    for url in urls.iter() {
        let table = load_table(&mut db, url)?;
        println!(
            "[{}] {}{}",
            table.symbol,
            table.name,
            table
                .last_modified
                .and_then(|v| DateTime::from_timestamp(v, 0))
                .map(|v| format!(" ({})", v.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S")))
                .unwrap_or_default()
        );

        if let Some(beatoraja) = &config.beatoraja {
            table.write_bmt(beatoraja)?;
        }
    }

    Ok(())
}
