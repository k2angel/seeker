use anyhow::Result;
use std::path::Path;
use tempfile::tempdir;

use seeker_core::database::Database;
use seeker_core::import::find_song_dirs;
use seeker_core::import::parser;
use seeker_core::model::song::HasMetadata;
use seeker_core::model::{SearchExpr, SearchTerm};

mod wrap;

#[test]
fn import_directory_test() -> Result<()> {
    let library = tempdir()?;
    let db_path = library.path().join("library.db");
    let mut db = Database::open(&db_path)?;
    db.create_schema()?;

    for (directory, files) in find_song_dirs(Path::new("tests/data/hitsugi")) {
        wrap::import_directory_with_default(&mut db, &library.path(), &directory, &files)?;
    }

    assert_eq!(db.count_songs(None)?, 1);
    assert_eq!(db.count_charts(None)?, 73);

    let entries: Vec<_> = std::fs::read_dir(library.path())?
        .filter_map(Result::ok)
        .collect();

    assert!(entries.len() >= 2);

    Ok(())
}

#[test]
fn import_chart_test() -> Result<()> {
    let library = tempdir()?;
    let path = library.path().join("library.db");
    let mut db = Database::open(&path)?;
    db.create_schema()?;

    for (directory, files) in find_song_dirs(Path::new("tests/data/hitsugi")) {
        wrap::import_directory_with_default(&mut db, &library.path(), &directory, &files)?;
    }

    let song = db.get_song(1)?;
    let expr = &SearchExpr::And(vec![SearchTerm::parse("私の狂気")]);
    let charts = db.search_charts(Some(expr))?;

    for chart in charts {
        let path = song.get_path(&library.path()).join(&chart.filename);

        std::fs::remove_file(&path)?;
        db.remove_chart(chart.id)?;
    }

    for (directory, files) in find_song_dirs(Path::new("tests/data/hitsugi")) {
        wrap::import_directory_with_default(&mut db, &library.path(), &directory, &files)?;
    }

    assert_eq!(db.count_songs(None)?, 1);

    Ok(())
}

#[test]
fn parse_chart_test() {
    let chart = wrap::parse_chart_with_sjis(&Path::new("tests/data/-.bms").to_path_buf()).unwrap();

    println!("{:#?}", chart);

    assert!(!chart.title.is_empty());
    assert!(!chart.md5.expect("REASON").is_empty());
    assert!(!chart.sha256.is_empty());

    assert_eq!(chart.title, "code:0000");
    assert_eq!(chart.artist, "-");
}

#[test]
fn build_song_test() {
    let dir = Path::new("tests/data/hitsugi");

    let mut charts = Vec::new();

    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if matches!(
                ext.to_ascii_lowercase().as_str(),
                "bms" | "bme" | "bml" | "pms"
            ) {
                charts.push(wrap::parse_chart_with_sjis(&path).unwrap());
            }
        }
    }

    let song = parser::build_song(&charts);

    println!("{:#?}", song);
    assert!(!song.title.is_empty());
}
