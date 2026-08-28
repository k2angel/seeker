use anyhow::Result;
use std::path::{Path, PathBuf};

use seeker::database::Database;
use seeker::import::import_directory;
use seeker::import::parser::parse_chart;
use seeker::model::Chart;
use seeker::model::import::{ImportEncoding, ImportOption, ImportResult};

pub fn parse_chart_with_sjis(path: &Path) -> Result<Chart> {
    Ok(parse_chart(path, &ImportEncoding::ShiftJis)?)
}

pub fn import_directory_with_default(
    db: &mut Database,
    library: &Path,
    root: &Path,
    files: &[PathBuf],
) -> Result<ImportResult> {
    let options = ImportOption {
        ..Default::default()
    };

    Ok(import_directory(db, library, root, files, options)?)
}
