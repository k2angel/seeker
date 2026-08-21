use anyhow::{Result, bail};
use dialoguer::MultiSelect;
use std::io::Write;
use std::ops::Index;

use crate::cli::RemoveArgs;
use crate::database::Database;
use crate::model::{Config, Song};
use crate::utils;

fn confirm_remove(items: &[String]) -> Result<Vec<usize>> {
    match items.len() {
        0 => bail!("No matching items found."),
        1 => {
            print!("Really remove 1 items from the library? (Yes/no) > ",);
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_ascii_lowercase().as_str() {
                "n" | "no" => bail!("Cancelled"),
                _ => Ok(vec![0]),
            }
        }
        count => {
            print!(
                "Really remove {} items from the library? (yes/no/Select) > ",
                count
            );
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_ascii_lowercase().as_str() {
                "y" | "yes" => Ok((0..count).collect()),
                "n" | "no" => bail!("Cancelled"),
                _ => {
                    let selections = MultiSelect::new()
                        .with_prompt("Select items")
                        .items(items)
                        .interact()?;

                    Ok(selections)
                }
            }
        }
    }
}

pub fn run(config: &Config, args: RemoveArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let expr = args.search.expr();

    if args.song {
        let songs = db.search_songs(expr.as_ref())?;
        let items: Vec<_> = songs
            .iter()
            .map(|song| format!("{} - {}", song.artist, song.title))
            .collect();

        for item in items.iter() {
            println!("{}", item);
        }

        let selections = confirm_remove(&items)?;

        for index in selections {
            let song_row = &songs[index];
            let song_id = song_row.id;
            let song = Song::try_from(song_row)?;

            std::fs::remove_dir_all(song.library_dir(&config.directory))?;
            db.remove_song(song_id)?;
        }
    } else {
        let charts = db.search_charts(expr.as_ref())?;
        let songs = utils::song_map(db.detail_songs(charts.iter().map(|c| c.song_id))?)?;
        let items: Vec<String> = charts
            .iter()
            .map(|chart| {
                let song = &songs[&chart.song_id];

                format!(
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
                )
            })
            .collect();

        for item in items.iter() {
            println!("{}", item)
        }

        let selections = confirm_remove(&items)?;

        for index in selections {
            let chart = charts.index(index);
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
