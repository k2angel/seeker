use anyhow::Result;

use crate::database::Database;
use crate::model::Config;
use crate::model::table::HasTimestamp;
use crate::table::load::load_table;

pub fn run(config: &Config, query: Option<String>) -> Result<()> {
    let mut db = Database::open(&config.library)?;
    db.create_schema()?;

    let tables = db.get_tables(query)?;

    for table in tables {
        let table_new = load_table(&mut db, &table.header_url)?;

        if table.sha256 != table_new.sha256 {
            println!(
                "[{}] {}{}{}",
                table_new.symbol,
                table_new.name,
                table
                    .get_timestamp()
                    .as_ref()
                    .map(|v| format!(" ({})", v))
                    .unwrap_or_default(),
                table_new
                    .get_timestamp()
                    .as_ref()
                    .map(|v| format!(" -> ({})", v))
                    .unwrap_or_default()
            );

            if let Some(beatoraja) = &config.beatoraja {
                table_new.write_bmt(beatoraja)?;
            }
        }
    }

    Ok(())
}
