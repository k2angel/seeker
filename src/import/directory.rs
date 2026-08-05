use anyhow::{Result, bail};
use fs_extra;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::database::Database;
use crate::import::parser::{build_song, parse_chart};
use crate::{model, utils};

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    let options = fs_extra::dir::CopyOptions {
        copy_inside: true,
        overwrite: true,
        ..Default::default()
    };

    if dst.exists() {
        let entries: Vec<_> = std::fs::read_dir(src)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<std::io::Result<_>>()?;

        fs_extra::copy_items(&entries, dst, &options)?;
    } else {
        fs_extra::dir::copy(src, dst, &options)?;
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
                            "bms" | "bme" | "bml" | "pms" | "bmw"
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
    root: &PathBuf,
    files: &[PathBuf],
) -> Result<model::ImportResult> {
    let mut charts = Vec::new();
    let mut charts_due = Vec::new();

    for file in files {
        let chart = parse_chart(&file)?;

        if !db.exists_chart(&chart.sha256)? {
            charts.push(chart);
        } else {
            charts_due.push(chart);
        }
    }

    let song = if !charts.is_empty() {
        let mut song = build_song(&charts);

        if song.title.is_empty() || song.artist.is_empty() {
            bail!("There may be multiple music tracks in the directory.");
        }

        let songs = db.find_songs(&song)?;
        let tx = db.transaction()?;

        let song_id = match songs.len() {
            0 => Database::insert_song(&tx, &song)?,
            1 => {
                song = songs.into_iter().next().unwrap();
                song.id.unwrap()
            }
            _ => todo!(),
        };

        for chart in &charts {
            Database::insert_chart(&tx, song_id, chart)?;
        }

        copy_dir(root, &song.library_dir(library))?;
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
