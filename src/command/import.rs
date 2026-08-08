use anyhow::Result;
use std::path::Path;
use tempfile::tempdir;

use crate::database::Database;
use crate::import::{extract_archive, find_song_dirs, import_directory};
use crate::model::Config;

pub fn run(config: &Config, path: &Path) -> Result<()> {
    let mut db = Database::open(&config.library)?;
    db.create_schema()?;

    let tmp;
    let root = if path.is_dir() {
        path
    } else {
        tmp = tempdir()?;
        extract_archive(path, tmp.path())?;
        tmp.path()
    };

    let song_dirs = find_song_dirs(root);

    for (i, (directory, files)) in song_dirs.clone().into_iter().enumerate() {
        println!(
            "\x1b[1;34m{} ({} items)\x1b[0m",
            directory.display(),
            files.len()
        );

        let result = import_directory(&mut db, &config.directory, &directory, &files)?;

        println!(
            "\x1b[33m{} - {}\x1b[m",
            result.song.artist, result.song.title
        );

        for chart in result.charts.iter() {
            println!(
                " - {}{} - {}{}",
                chart.artist,
                chart
                    .sub_artist
                    .as_ref()
                    .map(|s| format!(" / {}", s))
                    .unwrap_or_default(),
                chart.title,
                chart
                    .subtitle
                    .as_ref()
                    .map(|s| format!(" {}", s))
                    .unwrap_or_default(),
            );
        }

        for chart in result.charts_due.iter() {
            println!(
                " - \x1b[2m{}{} - {}{}\x1b[0m",
                chart.artist,
                chart
                    .sub_artist
                    .as_ref()
                    .map(|s| format!(" / {}", s))
                    .unwrap_or_default(),
                chart.title,
                chart
                    .subtitle
                    .as_ref()
                    .map(|s| format!(" {}", s))
                    .unwrap_or_default(),
            );
        }

        if i + 1 < song_dirs.len() {
            println!();
        }
    }

    Ok(())
}
