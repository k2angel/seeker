use anyhow::{Result, bail};
use filetime::{FileTime, set_file_mtime};
use std::collections::HashSet;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::database::Database;
use crate::import::parser::{build_song, parse_chart};
use crate::model::import::{ImportOption, ImportResult};
use crate::model::song::{HasMetadata, Song};

fn transfer_dir(src: &Path, dst: &Path, move_files: bool) -> Result<()> {
    let options = fs_extra::dir::CopyOptions {
        copy_inside: true,
        overwrite: true,
        ..Default::default()
    };

    let entries: Vec<_> = WalkDir::new(src)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.into_path())
        .collect();

    let mtimes: Vec<_> = entries
        .iter()
        .filter(|path| path != &src)
        .filter_map(|path| {
            let modified = std::fs::metadata(path).ok()?.modified().ok()?;
            let relative = path.strip_prefix(src).ok()?.to_owned();

            Some((relative, FileTime::from_system_time(modified)))
        })
        .collect();

    if dst.exists() {
        if move_files {
            fs_extra::move_items(&entries, dst, &options)?;
        } else {
            fs_extra::copy_items(&entries, dst, &options)?;
        }
    } else {
        if move_files {
            fs_extra::dir::move_dir(src, dst, &options)?;
        } else {
            fs_extra::dir::copy(src, dst, &options)?;
        }
    }

    for (relative, mtime) in mtimes {
        set_file_mtime(dst.join(relative), mtime)?;
    }

    Ok(())
}

pub fn find_song_dirs(root: &Path) -> Vec<(PathBuf, Vec<PathBuf>)> {
    let mut result = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        let dir = entry.path();

        let files: Vec<PathBuf> = std::fs::read_dir(dir)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|ext| {
                        matches!(
                            ext.to_ascii_lowercase().as_str(),
                            "bms" | "bme" | "bml" | "pms" | "bmw" | "bmson"
                        )
                    })
            })
            .collect();

        if result.iter().any(|(song_dir, _)| dir.starts_with(song_dir)) {
            continue;
        }

        if !files.is_empty() {
            result.push((dir.to_path_buf(), files));
        }
    }

    result
}

pub fn import_directory(
    db: &mut Database,
    library: &Path,
    root: &Path,
    files: &[PathBuf],
    option: &ImportOption,
) -> Result<ImportResult> {
    let mut charts = Vec::new();
    let mut charts_due = Vec::new();
    let mut chart_hashes = HashSet::new();

    for file in files {
        let chart = match catch_unwind(AssertUnwindSafe(|| parse_chart(file, &option.encoding))) {
            Ok(Ok(chart)) => chart,
            Ok(Err(err)) => {
                eprintln!("Warning: failed to parse {}: {}", file.display(), err);
                continue;
            }
            Err(_) => {
                eprintln!("Warning: parser panicked for {}", file.display());
                continue;
            }
        };

        if !chart_hashes.insert(chart.sha256.clone()) {
            continue;
        }

        if !db.exists_chart(&chart.sha256)? {
            charts.push(chart);
        } else {
            charts_due.push(chart);
        }
    }

    let song = if !charts.is_empty() {
        let mut song = build_song(&charts);

        if song.title.is_empty() {
            dbg!(charts.iter().map(|c| c.title.as_str()).collect::<Vec<_>>());
            dbg!(&song.title);
            bail!("There may be multiple music tracks in the directory.");
        }

        let result = db.find_song(&song)?;

        if !option.dry_run {
            let tx = db.transaction()?;

            let (song_id, exists) = match result {
                None => (Database::insert_song(&tx, &song)?, false),
                Some(result) => {
                    song = Song {
                        title: result.title,
                        artist: result.artist,
                        ..song
                    };

                    (result.id, true)
                }
            };

            for chart in charts.iter() {
                let filename = chart.filename.to_string_lossy();

                match Database::find_chart_by_filename(&tx, song_id, &filename)? {
                    Some(row) if row.modified_at < chart.modified_at => {
                        Database::update_chart(&tx, row.id, chart)?
                    }
                    Some(_) => {}
                    None => Database::insert_chart(&tx, song_id, chart)?,
                }
            }

            if exists {
                Database::update_song_resources(&tx, song_id)?;
            }

            transfer_dir(root, &song.get_path(library), option.r#move)?;
            tx.commit()?;
        }

        song
    } else {
        build_song(&charts_due)
    };

    Ok(ImportResult {
        song,
        charts,
        charts_due,
    })
}
