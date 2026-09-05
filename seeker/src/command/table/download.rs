use anyhow::Result;

use seeker_core::database::Database;
use seeker_core::model::{Config, SearchExpr, SearchTerm};
use seeker_core::table::parser::parse_data;
use seeker_core::utils::{is_url, tokenize};

use crate::cli::DownloadArgs;

fn print_url(title: &str, url: &str, detail: bool) {
    if detail {
        print!("{} - ", title);
    }

    println!("{url}");
}

pub fn run(config: Config, args: DownloadArgs) -> Result<()> {
    let db = Database::open(&config.library)?;
    db.create_schema()?;

    let tables = db.get_tables(args.query)?;

    for table in tables {
        let data = parse_data(&table.data)?;

        for chart in data {
            let Some(hash) = chart.sha256.as_deref().or(chart.md5.as_deref()) else {
                continue;
            };

            if db.exists_chart(hash)? {
                continue;
            }

            let push_url = args.url || (!args.url && !args.url_diff);
            let push_url_diff = args.url_diff || (!args.url && !args.url_diff);

            let skip_url = chart
                .org_md5
                .as_deref()
                .map_or(Ok(false), |org_md5| db.exists_chart(org_md5))?;

            if push_url
                && !skip_url
                && let Some(url) = chart.url.filter(|u| is_url(u))
            {
                if chart.artist.is_some() && chart.url_diff.as_deref().is_some_and(is_url) {
                    let expr = &SearchExpr::And(vec![
                        SearchTerm {
                            field: Some("title".to_string()),
                            value: tokenize(&chart.title)[0].value.clone(),
                        },
                        SearchTerm {
                            field: Some("artist".to_string()),
                            value: tokenize(&chart.artist.unwrap())[0].value.clone(),
                        },
                    ]);

                    if db.count_songs(Some(expr))? != 1 {
                        print_url(&chart.title, &url, args.detail);
                    }
                } else {
                    print_url(&chart.title, &url, args.detail);
                }
            }

            if push_url_diff && let Some(url_diff) = chart.url_diff.filter(|u| is_url(u)) {
                print_url(&chart.title, &url_diff, args.detail);
            }
        }
    }

    Ok(())
}
