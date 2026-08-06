use anyhow::{Ok, Result};
use clap::Parser;

use seeker::cli::{Cli, Command};
use seeker::command;
use seeker::config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load()?;

    match cli.command {
        Command::Import { path } => {
            for (i, p) in path.clone().into_iter().enumerate() {
                command::import::run(&config, &p)?;

                if i + 1 < path.len() {
                    println!();
                }
            }

            Ok(())
        }

        Command::List(args) => command::list::run(&config, args),
        Command::Remove(args) => command::remove::run(&config, args),
        Command::Stats(args) => command::stats::run(&config, args),
    }
}
