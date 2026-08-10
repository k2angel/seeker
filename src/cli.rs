use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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

#[derive(Debug, Clone, Copy)]
pub enum SearchField {
    All,
    Artist,
    Title,
}

#[derive(Debug, Clone)]
pub struct SearchTerm {
    pub field: SearchField,
    pub value: String,
}

impl SearchTerm {
    pub fn parse(query: &str) -> Self {
        if let Some(value) = query.strip_prefix("artist:") {
            return Self {
                field: SearchField::Artist,
                value: value.to_owned(),
            };
        }

        if let Some(value) = query.strip_prefix("title:") {
            return Self {
                field: SearchField::Title,
                value: value.to_owned(),
            };
        }

        Self {
            field: SearchField::All,
            value: query.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SearchExpr {
    And(Vec<SearchTerm>),
    Or(Vec<Vec<SearchTerm>>),
}

#[derive(Args)]
pub struct SearchArgs {
    pub query: Option<Vec<String>>,
}

impl SearchArgs {
    pub fn expr(&self) -> Option<SearchExpr> {
        let query = self.query.as_ref()?;

        let groups: Vec<Vec<SearchTerm>> = query
            .split(|query| query == "OR")
            .map(|group| group.iter().map(|query| SearchTerm::parse(query)).collect())
            .filter(|group: &Vec<SearchTerm>| !group.is_empty())
            .collect();

        match groups.len() {
            0 => None,
            1 => Some(SearchExpr::And(groups.into_iter().next().unwrap())),
            _ => Some(SearchExpr::Or(groups)),
        }
    }
}
