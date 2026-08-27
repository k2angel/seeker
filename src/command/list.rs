use anyhow::Result;

use crate::cli::ListArgs;
use crate::database::Database;
use crate::model::{Config, Song};
use crate::utils;

pub fn run(config: &Config, args: ListArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let expr = args.search.expr();

    if args.song {
        let songs = db.search_songs(expr.as_ref())?;

        for song_row in songs {
            let song = Song::try_from(song_row)?;
            if args.path {
                println!("{}", song.library_dir(&config.directory).display())
            } else {
                println!("{} - {}", song.artist, song.title);
            }
        }
    } else {
        let charts = db.search_charts(expr.as_ref())?;
        let songs = utils::song_map(db.detail_songs(charts.iter().map(|c| c.song_id))?)?;

        for chart in charts.iter() {
            let song = &songs[&chart.song_id];

            if args.path {
                println!(
                    "{}",
                    song.library_dir(&config.directory)
                        .join(&chart.filename)
                        .display()
                );
            } else {
                println!(
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
                );
            }
        }
    }

    Ok(())
}
