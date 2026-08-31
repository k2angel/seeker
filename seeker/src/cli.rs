use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_complete::aot::Shell;
use std::path::PathBuf;

use seeker_core::model::import::Encoding;
use seeker_core::model::{SearchExpr, SearchTerm};

#[derive(Parser)]
#[command(version, about = "BMS library manager.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Import BMS files into the library.
    Import(ImportArgs),

    /// List songs or charts.
    #[command(visible_alias = "ls")]
    List(ListArgs),

    /// Remove songs or charts.
    #[command(visible_alias = "rm")]
    Remove(RemoveArgs),

    /// Show library statics.
    Stats(StatsArgs),

    /// Open in STELLAVERSE IR.
    Open(OpenArgs),

    /// Generate shell completion.
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },

    #[command(subcommand)]
    Table(TableSubcommand),
}

#[derive(Subcommand)]
pub enum TableSubcommand {
    /// Import tables.
    Import { url: Vec<String> },

    /// List tables.
    #[command(visible_alias = "ls")]
    List(TableListArgs),

    /// Remove tables.
    #[command(visible_alias = "rm")]
    Remove { query: Option<String> },

    /// Update tables.
    Update { query: Option<String> },

    /// Show chart url or url_diff.
    Download(DownloadArgs),
}

#[derive(Clone, ValueEnum)]
pub enum ImportEncoding {
    ShiftJis,
    Big5,
    EucKr,
    Auto,
}

#[derive(Args)]
pub struct ImportArgs {
    /// Files or directories to import.
    pub path: Vec<PathBuf>,

    /// Move files.
    #[arg(short, long)]
    pub r#move: bool,

    /// Select file encoding.
    #[arg(short, long, value_enum)]
    pub encoding: Option<ImportEncoding>,

    /// Do not import.
    #[arg(long)]
    pub dry_run: bool,
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

    /// Do not ask when removing items.
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct StatsArgs {
    #[command(flatten)]
    pub search: SearchArgs,
}

#[derive(Args)]
pub struct OpenArgs {
    #[command(flatten)]
    pub search: SearchArgs,

    /// Open in BMS Score Viewer.
    #[arg(long)]
    pub viewer: bool,
}

#[derive(Args)]
pub struct TableListArgs {
    pub query: Option<String>,

    /// Show table header_url.
    #[arg(short, long)]
    pub url: bool,
}

#[derive(Args)]
pub struct DownloadArgs {
    pub query: Option<String>,

    /// Show chart title.
    #[arg(long)]
    pub detail: bool,

    /// Show chart utl only.
    #[arg(long)]
    pub url: bool,

    /// Show chart url_diff only.
    #[arg(long)]
    pub url_diff: bool,
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

impl From<ImportEncoding> for Encoding {
    fn from(value: ImportEncoding) -> Self {
        match value {
            ImportEncoding::Auto => Self::Auto,
            ImportEncoding::ShiftJis => Self::ShiftJis,
            ImportEncoding::Big5 => Self::Big5,
            ImportEncoding::EucKr => Self::EucKr,
        }
    }
}
