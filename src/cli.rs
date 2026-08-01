use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Import {
        path: Vec<PathBuf>,
    },

    #[command(visible_alias = "ls")]
    List(ListArgs),

    #[command(visible_alias = "rm")]
    Remove(RemoveArgs),

    Stats {},
}

#[derive(clap::Args)]
pub struct ListArgs {
    #[arg(short, long)]
    pub song: bool,

    #[arg(short, long)]
    pub path: bool,

    pub query: Option<String>,
}

#[derive(clap::Args)]
pub struct RemoveArgs {
    #[arg(short, long)]
    pub song: bool,

    pub query: Option<String>,
}
