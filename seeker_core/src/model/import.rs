use crate::model::{Chart, Song};

#[derive(Default)]
pub enum Encoding {
    #[default]
    ShiftJis,
    Big5,
    EucKr,
    Auto,
}

#[derive(Default)]
pub struct ImportOption {
    pub r#move: bool,
    pub dry_run: bool,
    pub encoding: Encoding,
}

pub struct ImportResult {
    pub song: Song,
    pub charts: Vec<Chart>,
    pub charts_due: Vec<Chart>,
}
