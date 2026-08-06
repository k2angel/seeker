use anyhow::Result;

use crate::cli::StatsArgs;
use crate::database::Database;
use crate::model::Config;

pub fn run(config: &Config, args: StatsArgs) -> Result<()> {
    let db = Database::open(&config.database)?;
    db.create_schema()?;

    let expr = args.search.expr();

    println!("Songs : {:?}", db.count_songs(expr.as_ref())?);
    println!("Charts: {:?}", db.count_charts(expr.as_ref())?);

    Ok(())
}
