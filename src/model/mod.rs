mod bmt;
mod chart;
mod config;
mod search;
mod song;
pub mod table;

pub use chart::{Chart, ChartRow};
pub use config::Config;
pub use search::{SearchArgs, SearchExpr, SearchTerm};
pub use song::{Song, SongRow};

pub struct ImportResult {
    pub song: Song,
    pub charts: Vec<Chart>,
    pub charts_due: Vec<Chart>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Separator {
    None,
    Space,
    Split,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
    pub separator: Separator,
}
