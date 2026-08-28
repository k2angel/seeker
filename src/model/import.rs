use clap::ValueEnum;

use crate::model::{Chart, Song};

#[derive(Clone, ValueEnum)]
pub enum ImportEncoding {
    ShiftJis,
    Big5,
    EucKr,
    Auto,
}

pub struct ImportOption {
    pub r#move: bool,
    pub dry_run: bool,
    pub encoding: ImportEncoding,
}

impl Default for ImportOption {
    fn default() -> Self {
        Self {
            r#move: false,
            dry_run: false,
            encoding: ImportEncoding::ShiftJis,
        }
    }
}

pub struct ImportResult {
    pub song: Song,
    pub charts: Vec<Chart>,
    pub charts_due: Vec<Chart>,
}
