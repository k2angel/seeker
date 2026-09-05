use anyhow::Result;

use seeker_core::database::Database;
use seeker_core::model::Config;
use seeker_core::model::song::HasMetadata;
use seeker_core::utils;

use crate::cli::ListArgs;

pub fn run(config: Config, args: ListArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let expr = args.search.expr();

    if args.song {
        let songs = db.search_songs(expr.as_ref())?;

        for song in songs {
            if args.path {
                println!("{}", song.get_path(&config.directory).display())
            } else {
                println!("{} - {}", song.artist, song.title);
            }
        }
    } else {
        let charts = db.search_charts(expr.as_ref())?;
        let songs = utils::song_map(db.get_songs(charts.iter().map(|c| c.song_id))?)?;

        for chart in charts.iter() {
            let song = &songs[&chart.song_id];

            if args.path {
                println!(
                    "{}",
                    song.get_path(&config.directory)
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
