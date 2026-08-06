use anyhow::{Result, bail};
use std::io::{self, Write};

use crate::cli::RemoveArgs;
use crate::database::Database;
use crate::model::Config;

fn confirm_remove(count: usize) -> Result<()> {
    if count == 0 {
        bail!("No matching items found.");
    }

    print!(
        "Really remove {} items from the library? (yes/No) > ",
        count
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" => Ok(()),
        _ => bail!("Cancelled"),
    }
}

pub fn run(config: &Config, args: RemoveArgs) -> Result<()> {
    let db = Database::open(&config.database)?;
    db.create_schema()?;

    let expr = args.search.expr();

    if args.song {
        let songs = db.search_songs(expr.as_ref())?;

        for song in songs.iter() {
            println!("{} - {}", song.artist, song.title);
        }

        confirm_remove(songs.len())?;

        for song in songs.iter() {
            std::fs::remove_dir_all(song.library_dir(&config.library))?;
            db.remove_song(song.id.unwrap())?;
        }
    } else {
        let charts = db.search_charts(expr.as_ref())?;
        let songs = db.detail_songs(charts.iter().filter_map(|c| c.song_id))?;

        for chart in charts.iter() {
            let song = &songs[&chart.song_id.unwrap()];

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
            let song = &songs[&chart.song_id.unwrap()];
            let song_id = chart.song_id.unwrap();
            let path = song.library_dir(&config.library).join(&chart.filename);

            std::fs::remove_file(&path)?;
            db.remove_chart(chart.id.unwrap())?;

            if db.count_song_charts(song_id)? == 0 {
                std::fs::remove_dir_all(song.library_dir(&config.library))?;
                db.remove_song(song_id)?;
            }
        }
    }

    Ok(())
}
