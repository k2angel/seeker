use anyhow::{Ok, Result, bail};
use dialoguer::MultiSelect;
use std::io::Write;

use crate::cli::OpenArgs;
use crate::database::Database;
use crate::model::{ChartRow, Config};
use crate::utils::song_map;

fn confirm_open(count: usize, items: Vec<String>) -> Result<Vec<usize>> {
    match count {
        0 => bail!("No matching items found."),
        1 => {
            print!("Really open {} items from the library? (Yes/no) > ", count);
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_ascii_lowercase().as_str() {
                "n" | "no" => bail!("Cancelled"),
                _ => Ok(vec![0]),
            }
        }
        _ => {
            print!(
                "Really open {} items from the library? (yes/no/Select) > ",
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
                        .with_prompt("Select charts")
                        .items(&items)
                        .interact()?;

                    Ok(selections)
                }
            }
        }
    }
}

pub fn run(config: &Config, args: OpenArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let expr = args.search.expr();

    let charts: Vec<_> = db
        .search_charts(expr.as_ref())?
        .into_iter()
        .filter(|chart| chart.md5.is_some())
        .collect();

    let songs = song_map(db.detail_songs(charts.iter().map(|c| c.song_id))?)?;

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
        println!("{}", item);
    }

    let selections = confirm_open(items.len(), items)?;

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
