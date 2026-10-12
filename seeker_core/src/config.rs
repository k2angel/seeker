use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

use crate::model::Config;

fn config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("io.github", "k2angel", "seeker")
        .context("failed to determine config directory")?;

    Ok(proj_dirs.config_dir().join("config.toml"))
}

pub fn load() -> Result<Config> {
    let path = config_path()?;
    let r = fs::read(&path)?;

    Ok(toml::from_slice(&r)?)
}
