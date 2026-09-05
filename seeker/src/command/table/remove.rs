use anyhow::Result;

use seeker_core::database::Database;
use seeker_core::model::Config;
use seeker_core::model::table::Table;

use crate::utils::confirm_input;

pub fn run(config: Config, query: Option<String>) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let tables: Vec<_> = db.get_tables(query)?;
    let items: Vec<String> = tables.iter().map(|table| table.name.clone()).collect();

    for item in items.iter() {
        println!("{}", item);
    }

    let selections = confirm_input("remove", &items)?;

    for index in selections {
        let table = &tables[index];
        db.remove_table(table.id)?;

        if let Some(beatoraja) = &config.beatoraja {
            let bmt = Table::try_from(table)?.to_bmt()?;
            bmt.remove(beatoraja)?;
        }
    }

    Ok(())
}
