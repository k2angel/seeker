use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
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

#[derive(Args)]
pub struct ListArgs {
    #[command(flatten)]
    pub search: SearchArgs,

    #[arg(short, long)]
    pub song: bool,

    #[arg(short, long)]
    pub path: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[command(flatten)]
    pub search: SearchArgs,

    #[arg(short, long)]
    pub song: bool,
}

#[derive(Args)]
pub struct StatsArgs {
    #[command(flatten)]
    pub search: SearchArgs,
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

    Stats(StatsArgs),
}
