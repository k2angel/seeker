use anyhow::Result;

use seeker_core::database::Database;
use seeker_core::model::Config;
use seeker_core::model::song::HasMetadata;
use seeker_core::utils::song_map;

use crate::cli::RemoveArgs;
use crate::utils::confirm_input;

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

        let selections = if args.force {
            (0..items.len()).collect()
        } else {
            confirm_input("remove", &items)?
        };

        for index in selections {
            let song = &songs[index];
            let song_id = song.id;

            std::fs::remove_dir_all(song.library_dir(&config.directory))?;
            db.remove_song(song_id)?;
        }
    } else {
        let charts = db.search_charts(expr.as_ref())?;
        let songs = song_map(db.get_songs(charts.iter().map(|c| c.song_id))?)?;
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
                        .map(|s| format!(" {}", s))
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

        let selections = if args.force {
            (0..items.len()).collect()
        } else {
            confirm_input("remove", &items)?
        };

        for index in selections {
            let chart = &charts[index];
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
