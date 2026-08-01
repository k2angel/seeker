use anyhow::{Result, bail};
use sevenz_rust;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use unrar;
use zip::ZipArchive;

fn extract_zip(path: &Path, output: &Path) -> Result<()> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let path = output.join(entry.mangled_name());

        if entry.is_dir() {
            fs::create_dir_all(&path)?;
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut out = File::create(&path)?;
            io::copy(&mut entry, &mut out)?;
        }
    }

    Ok(())
}

fn extract_7z(path: &Path, output: &Path) -> Result<()> {
    sevenz_rust::decompress_file(path, output)?;

    Ok(())
}

fn extract_rar(path: &Path, output: &Path) -> Result<()> {
    let mut archive = unrar::Archive::new(path).open_for_processing().unwrap();

    while let Some(header) = archive.read_header()? {
        archive = if header.entry().is_file() {
            header.extract_with_base(output)?
        } else {
            header.skip()?
        };
    }

    Ok(())
}

pub fn extract_archive(path: &Path, output: &Path) -> Result<()> {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("zip") => extract_zip(path, output),
        Some("7z") => extract_7z(path, output),
        Some("rar") => extract_rar(path, output),
        _ => bail!("unsupported archive"),
    }
}
