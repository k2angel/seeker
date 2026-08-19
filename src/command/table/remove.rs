use anyhow::{Result, bail};
use std::io::{self, Write};

use crate::database::Database;
use crate::model::Config;
use crate::model::table::Table;

fn confirm_remove(count: usize) -> Result<()> {
    if count == 0 {
        bail!("No matching items found.");
    }

    print!(
        "Really remove {} items from the library? (Yes/no) > ",
        count
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_ascii_lowercase().as_str() {
        "n" | "no" => bail!("Cancelled"),
        _ => Ok(()),
    }
}

pub fn run(config: &Config, query: Option<String>) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let tables = db.get_tables(query)?;

    for table in tables.iter() {
        println!("{}", table.name);
    }

    confirm_remove(tables.len())?;

    for table in tables {
        db.remove_table(table.id)?;

        if let Some(beatoraja) = &config.beatoraja {
            Table::try_from(table)?.remove_bmt(beatoraja)?;
        }
    }

    Ok(())
}
