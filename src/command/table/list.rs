use anyhow::Result;
use chrono::{DateTime, Local};

use crate::cli::TableListArgs;
use crate::database::Database;
use crate::model::Config;

pub fn run(config: &Config, args: TableListArgs) -> Result<()> {
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
                    .last_modified
                    .and_then(|v| DateTime::from_timestamp(v, 0))
                    .map(|v| format!(" ({})", v.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S")))
                    .unwrap_or_default()
            );
        }
    }

    Ok(())
}
