use anyhow::Result;

use crate::database::Database;
use crate::model::Config;

pub fn run(config: &Config) -> Result<()> {
    let db = Database::open(&config.database)?;
    db.create_schema()?;

    println!("Songs : {:?}", db.count_songs()?);
    println!("Charts: {:?}", db.count_charts()?);

    Ok(())
}
