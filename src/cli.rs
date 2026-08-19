use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::model::SearchArgs;

#[derive(Parser)]
#[command(version, about = "BMS library manager.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Import BMS files into the libarry.
    Import(ImportArgs),

    /// List songs or charts.
    #[command(visible_alias = "ls")]
    List(ListArgs),

    /// Remove songs or charts.
    #[command(visible_alias = "rm")]
    Remove(RemoveArgs),

    /// Show library statics.
    Stats(StatsArgs),

    #[command(subcommand)]
    Table(TableSubcommand),
}

#[derive(Subcommand)]
pub enum TableSubcommand {
    Import {
        url: Vec<String>,
    },

    #[command(visible_alias = "ls")]
    List(TableListArgs),

    #[command(visible_alias = "rm")]
    Remove {
        query: Option<String>,
    },

    Update {
        query: Option<String>,
    },
}

#[derive(Args)]
pub struct ImportArgs {
    /// Files or directories to import.
    pub path: Vec<PathBuf>,

    /// Move files.
    #[arg(short, long)]
    pub r#move: bool,
}

#[derive(Args)]
pub struct ListArgs {
    #[command(flatten)]
    pub search: SearchArgs,

    /// List songs instead of charts.
    #[arg(short, long)]
    pub song: bool,

    /// Print paths instead of song/chart information.
    #[arg(short, long)]
    pub path: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[command(flatten)]
    pub search: SearchArgs,

    /// Remove songs instead of charts.
    #[arg(short, long)]
    pub song: bool,
}

#[derive(Args)]
pub struct StatsArgs {
    #[command(flatten)]
    pub search: SearchArgs,
}

#[derive(Args)]
pub struct TableListArgs {
    pub query: Option<String>,

    #[arg(short, long)]
    pub url: bool,
}
