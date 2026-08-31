use anyhow::Result;
use clap::Parser;

mod cli;
mod command;
mod utils;

use crate::cli::{Cli, Command, TableSubcommand};
use seeker_core::config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load()?;

    match cli.command {
        Command::Import(args) => command::import::run(&config, args),
        Command::List(args) => command::list::run(&config, args),
        Command::Remove(args) => command::remove::run(&config, args),
        Command::Stats(args) => command::stats::run(&config, args),
        Command::Open(args) => command::open::run(&config, args),
        Command::Completion { shell } => command::completion::run(shell),

        Command::Table(command) => match command {
            TableSubcommand::Import { url } => command::table::import::run(&config, url),
            TableSubcommand::List(args) => command::table::list::run(&config, args),
            TableSubcommand::Remove { query } => command::table::remove::run(&config, query),
            TableSubcommand::Update { query } => command::table::update::run(&config, query),
            TableSubcommand::Download(args) => command::table::download::run(&config, args),
        },
    }
}
