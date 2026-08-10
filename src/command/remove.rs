use anyhow::{Result, bail};
use std::io::{self, Write};

use crate::cli::RemoveArgs;
use crate::database::Database;
use crate::model::{Config, Song};
use crate::utils;

fn confirm_remove(count: usize) -> Result<()> {
    if count == 0 {
        bail!("No matching items found.");
    }

    print!(
        "Really remove {} items from the library? (Yes/no) > ",
        count
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_ascii_lowercase().as_str() {
        "n" | "no" => bail!("Cancelled"),
        _ => Ok(()),
    }
}

pub fn run(config: &Config, args: RemoveArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let expr = args.search.expr();

    if args.song {
        let songs = db.search_songs(expr.as_ref())?;

        for song in songs.iter() {
            println!("{} - {}", song.artist, song.title);
        }

        confirm_remove(songs.len())?;

        for song_row in songs {
            let song_id = song_row.id;
            let song = Song::try_from(song_row)?;

            std::fs::remove_dir_all(song.library_dir(&config.directory))?;
            db.remove_song(song_id)?;
        }
    } else {
        let charts = db.search_charts(expr.as_ref())?;
        let songs = utils::song_map(db.detail_songs(charts.iter().map(|c| c.song_id))?)?;

        for chart in charts.iter() {
            let song = &songs[&chart.song_id];

            println!(
                "{}{} - {} - {}{}",
                chart.artist,
                chart
                    .sub_artist
                    .as_ref()
                    .map(|s| format!(" / {}", s))
                    .unwrap_or_default(),
                song.title,
                chart.title,
                chart
                    .subtitle
                    .as_ref()
                    .map(|s| format!(" {}", s))
                    .unwrap_or_default(),
            );
        }

        confirm_remove(charts.len())?;

        for chart in charts.iter() {
            let song = &songs[&chart.song_id];
            let song_id = chart.song_id;
            let path = song.library_dir(&config.directory).join(&chart.filename);

            std::fs::remove_file(&path)?;
            db.remove_chart(chart.id)?;

            if db.count_song_charts(song_id)? == 0 {
                std::fs::remove_dir_all(song.library_dir(&config.directory))?;
                db.remove_song(song_id)?;
            }
        }
    }

    Ok(())
}
