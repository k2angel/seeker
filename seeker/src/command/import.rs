use anyhow::Result;
use std::path::Path;

use seeker_core::database::Database;
use seeker_core::import::{copy_to_tmp, extract_to_tmp, find_song_dirs, import_directory};
use seeker_core::model::Config;
use seeker_core::model::import::ImportOption;
use seeker_core::utils::is_bms;

use crate::cli::ImportArgs;

pub fn run(config: Config, args: ImportArgs) -> Result<()> {
    let mut db = Database::open(&config.library)?;
    db.create_schema()?;

    let options = ImportOption {
        r#move: args.r#move,
        dry_run: args.dry_run,
        encoding: args.encoding.map(Into::into).unwrap_or_default(),
    };

    for (i, p) in args.path.clone().into_iter().enumerate() {
        if let Err(err) = main(&mut db, &config, &p, &options) {
            eprintln!("Error: {}", err);
        }

        if i + 1 < args.path.len() {
            println!();
        }
    }

    Ok(())
}

fn main(db: &mut Database, config: &Config, path: &Path, options: &ImportOption) -> Result<()> {
    let tmp;
    let root = if path.is_dir() {
        path
    } else if is_bms(path.extension()) {
        tmp = copy_to_tmp(path)?;
        tmp.path()
    } else {
        tmp = extract_to_tmp(path)?;
        tmp.path()
    };

    let song_dirs = find_song_dirs(root);

    for (i, (directory, files)) in song_dirs.clone().into_iter().enumerate() {
        println!(
            "\x1b[1;34m{} ({} items)\x1b[0m",
            directory.display(),
            files.len()
        );

        let result = match import_directory(db, &config.directory, &directory, &files, options) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

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
                    .map(|s| format!(" {}", s))
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
                    .map(|s| format!(" {}", s))
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
