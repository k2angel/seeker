use anyhow::Result;

use crate::cli::TableListArgs;
use seeker_core::database::Database;
use seeker_core::model::Config;
use seeker_core::model::table::HasTimestamp;

pub fn run(config: Config, args: TableListArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let tables = db.get_tables(args.query)?;

    for table in tables {
        if args.url {
            println!("{}", table.header_url)
        } else {
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
        }
    }

    Ok(())
}
