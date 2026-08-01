use anyhow::Result;
use std::{fs, path::Path};

use seeker::database::Database;
use seeker::import::parser::{build_song, parse_chart};

#[test]
fn database_insert_test() -> Result<()> {
    let path = Path::new(":memory:");
    let mut db = Database::open(path)?;
    db.create_schema()?;

    let mut charts = Vec::new();

    for entry in fs::read_dir("tests/data/hitsugi")? {
        let path = entry?.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if matches!(
                ext.to_ascii_lowercase().as_str(),
                "bms" | "bme" | "bml" | "pms"
            ) {
                charts.push(parse_chart(&path)?);
            }
        }
    }

    assert!(!charts.is_empty());

    let song = build_song(&charts);
    let tx = db.transaction()?;

    let song_id = Database::insert_song(&tx, &song)?;

    for chart in &charts {
        Database::insert_chart(&tx, song_id, chart)?;
    }

    tx.commit()?;

    Ok(())
}
