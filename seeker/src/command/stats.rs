use anyhow::Result;

use seeker_core::database::Database;
use seeker_core::model::Config;

use crate::cli::StatsArgs;

pub fn run(config: Config, args: StatsArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let expr = args.search.expr();

    println!("Songs : {:?}", db.count_songs(expr.as_ref())?);
    println!("Charts: {:?}", db.count_charts(expr.as_ref())?);

    Ok(())
}
