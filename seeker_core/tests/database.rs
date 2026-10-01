use anyhow::Result;
use std::{fs, path::Path};

use seeker_core::database::Database;
use seeker_core::import::parser::build_song;
use seeker_core::utils::is_bms;

mod wrap;

#[test]
fn database_insert_test() -> Result<()> {
    let path = Path::new(":memory:");
    let mut db = Database::open(path)?;
    db.create_schema()?;

    let mut charts = Vec::new();

    for entry in fs::read_dir("tests/data/hitsugi")? {
        let path = entry?.path();

        if is_bms(path.extension()) {
            charts.push(wrap::parse_chart_with_sjis(&path)?);
        }
    }

    assert!(!charts.is_empty());

    let song = build_song(&charts);
    let tx = db.transaction()?;

    let song_id = Database::insert_song(&tx, &song)?;

    for chart in charts.iter() {
        Database::insert_chart(&tx, song_id, chart)?;
    }

    tx.commit()?;

    Ok(())
}
