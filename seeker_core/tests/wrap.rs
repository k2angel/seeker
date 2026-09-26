use anyhow::Result;
use std::path::{Path, PathBuf};

use seeker_core::database::Database;
use seeker_core::import::import_directory;
use seeker_core::import::parser::parse_chart;
use seeker_core::model::Chart;
use seeker_core::model::import::{Encoding, ImportResult};

pub fn parse_chart_with_sjis(path: &PathBuf) -> Result<Chart> {
    Ok(parse_chart(path, &Encoding::ShiftJis)?)
}

pub fn import_directory_with_default(
    db: &mut Database,
    library: &Path,
    root: &Path,
    files: &Vec<PathBuf>,
) -> Result<ImportResult> {
    Ok(import_directory(
        db,
        library,
        root,
        files,
        &Default::default(),
    )?)
}
