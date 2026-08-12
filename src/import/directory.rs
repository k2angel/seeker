use anyhow::{Result, bail};
use fs_extra;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::database::Database;
use crate::import::parser::{build_song, parse_chart};
use crate::model;

fn transfer_dir(src: &Path, dst: &Path, move_files: bool) -> Result<()> {
    let options = fs_extra::dir::CopyOptions {
        copy_inside: true,
        overwrite: true,
        ..Default::default()
    };

    if dst.exists() {
        let entries: Vec<_> = std::fs::read_dir(src)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<std::io::Result<_>>()?;

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

        let files: Vec<PathBuf> = fs::read_dir(dir)
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
    move_files: bool,
) -> Result<model::ImportResult> {
    let mut charts = Vec::new();
    let mut charts_due = Vec::new();
    let mut chart_hashes = HashSet::new();

    for file in files {
        let chart = match parse_chart(file) {
            Ok(chart) => chart,
            Err(err) => {
                eprintln!("Warning: failed to parse {}: {}", file.display(), err);
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

        if song.title.is_empty() || song.artist.is_empty() {
            dbg!(charts.iter().map(|c| c.title.as_str()).collect::<Vec<_>>());
            dbg!(charts.iter().map(|c| c.artist.as_str()).collect::<Vec<_>>());
            dbg!(&song.title, &song.artist);
            bail!("There may be multiple music tracks in the directory.");
        }

        let result = db.find_song(&song)?;
        let tx = db.transaction()?;

        let (song_id, exists) = match result {
            None => (Database::insert_song(&tx, &song)?, false),
            Some(result) => {
                song = model::Song {
                    title: result.title,
                    artist: result.artist,
                    ..song
                };

                (result.id, true)
            }
        };

        for chart in &charts {
            Database::insert_chart(&tx, song_id, chart)?;
        }

        if exists {
            Database::rebuild_song_resources(&tx, song_id)?;
        }

        transfer_dir(root, &song.library_dir(library), move_files)?;
        tx.commit()?;

        song
    } else {
        build_song(&charts_due)
    };

    Ok(model::ImportResult {
        song,
        charts,
        charts_due,
    })
}
