use anyhow::Result;

use crate::cli::OpenArgs;
use seeker_core::database::Database;
use seeker_core::model::Config;
use seeker_core::utils::song_map;

use crate::utils::confirm_input;

pub fn run(config: &Config, args: OpenArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let expr = args.search.expr();

    let charts: Vec<_> = db
        .search_charts(expr.as_ref())?
        .into_iter()
        .filter(|chart| chart.md5.is_some())
        .collect();

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
        println!("{}", item);
    }

    let selections = confirm_input("open", &items)?;

    for index in selections {
        let chart = &charts[index];

        if args.viewer {
            open::that(format!(
                "https://bms-score-viewer.pages.dev/view?md5={}",
                chart.md5.as_deref().unwrap()
            ))?;
        } else {
            open::that(format!(
                "https://ir.stellabms.xyz/charts/{}",
                chart.md5.as_deref().unwrap()
            ))?;
        }
    }

    Ok(())
}
