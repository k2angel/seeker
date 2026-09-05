use anyhow::Result;

use seeker_core::database::Database;
use seeker_core::model::Config;
use seeker_core::model::table::HasTimestamp;
use seeker_core::table::load::load_table;

pub fn run(config: Config, urls: Vec<String>) -> Result<()> {
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
            let bmt = table.to_bmt()?;
            bmt.write(beatoraja)?;
        }
    }

    Ok(())
}
