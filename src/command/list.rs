use anyhow::Result;

use crate::cli::ListArgs;
use crate::database::Database;
use crate::model::Config;

pub fn run(config: &Config, args: ListArgs) -> Result<()> {
    let db = Database::open(&config.database)?;
    db.create_schema()?;

    if args.song {
        let songs = db.search_songs(args.query)?;

        for song in songs.iter() {
            if args.path {
                println!("{}", song.library_dir(&config.library).display())
            } else {
                println!("{} - {}", song.artist, song.title);
            }
        }
    } else {
        let charts = db.search_charts(args.query)?;
        let songs = db.detail_songs(charts.iter().filter_map(|c| c.song_id))?;

        for chart in charts.iter() {
            let song = &songs[&chart.song_id.unwrap()];

            if args.path {
                println!(
                    "{}",
                    song.library_dir(&config.library)
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
        }
    }

    Ok(())
}
