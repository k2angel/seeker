use anyhow::Result;

use crate::database::Database;
use crate::model::Config;
use crate::model::table::HasTimestamp;
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
                .get_timestamp()
                .as_ref()
                .map(|v| format!(" ({})", v))
                .unwrap_or_default()
        );

        if let Some(beatoraja) = &config.beatoraja {
            table.write_bmt(beatoraja)?;
        }
    }

    Ok(())
}
