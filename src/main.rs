use anyhow::Result;
use clap::Parser;

use seeker::cli::{Cli, Command};
use seeker::command;
use seeker::config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load()?;

    match cli.command {
        Command::Import(args) => command::import::run(&config, args),
        Command::List(args) => command::list::run(&config, args),
        Command::Remove(args) => command::remove::run(&config, args),
        Command::Stats(args) => command::stats::run(&config, args),
    }
}
