use anyhow::{Result, bail};
use dialoguer::MultiSelect;
use std::io::Write;

use crate::database::Database;
use crate::model::Config;
use crate::model::table::Table;

fn confirm_remove(items: &[String]) -> Result<Vec<usize>> {
    match items.len() {
        0 => bail!("No matching items found."),
        1 => {
            print!("Really remove 1 items from the library? (Yes/no) > ",);
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_ascii_lowercase().as_str() {
                "n" | "no" => bail!("Cancelled"),
                _ => Ok(vec![0]),
            }
        }
        count => {
            print!(
                "Really remove {} items from the library? (yes/no/Select) > ",
                count
            );
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_ascii_lowercase().as_str() {
                "y" | "yes" => Ok((0..count).collect()),
                "n" | "no" => bail!("Cancelled"),
                _ => {
                    let selections = MultiSelect::new()
                        .with_prompt("Select items")
                        .items(items)
                        .interact()?;

                    Ok(selections)
                }
            }
        }
    }
}

pub fn run(config: &Config, query: Option<String>) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let tables: Vec<_> = db.get_tables(query)?;
    let items: Vec<String> = tables.iter().map(|table| table.name.clone()).collect();

    for item in items.iter() {
        println!("{}", item);
    }

    let selections = confirm_remove(&items)?;

    for index in selections {
        let table = &tables[index];
        db.remove_table(table.id)?;

        if let Some(beatoraja) = &config.beatoraja {
            Table::try_from(table)?.remove_bmt(beatoraja)?;
        }
    }

    Ok(())
}
