use anyhow::{Context, Result, bail};
use filetime::{FileTime, set_file_mtime};
use sevenz_rust;
use std::fs::{self, File};
use std::io::{Read, copy};
use std::path::{Path, PathBuf};
use unrar;
use zip::ExtraField;
use zip::read::{ZipArchive, ZipFile};

fn entry_mtime<R: Read>(entry: &ZipFile<'_, R>) -> Option<FileTime> {
    for field in entry.extra_data_fields() {
        match field {
            // NTFS extra field
            ExtraField::Ntfs(ntfs) => {
                let (secs, nanos) = ntfs.modified_file_time().to_unix_time();

                return Some(FileTime::from_unix_time(secs, nanos));
            }

            // Extended Timestamp
            ExtraField::ExtendedTimestamp(ts) => {
                if let Some(sec) = ts.mod_time() {
                    return Some(FileTime::from_unix_time(sec as i64, 0));
                }
            }
        }
    }

    // DOS timestamp
    let dt: time::PrimitiveDateTime = entry.last_modified()?.try_into().ok()?;

    Some(FileTime::from_unix_time(
        dt.assume_utc().unix_timestamp(),
        dt.nanosecond(),
    ))
}

fn extract_zip(path: &Path, output: &Path) -> Result<()> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut mtimes: Vec<(PathBuf, FileTime)> = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let path = output.join(entry.mangled_name());

        if let Some(mtime) = entry_mtime(&entry) {
            mtimes.push((path.clone(), mtime));
        }

        if entry.is_dir() {
            fs::create_dir_all(&path)?;
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut out = File::create(&path)?;
            copy(&mut entry, &mut out)?;
        }
    }

    for (path, mtime) in mtimes {
        set_file_mtime(path, mtime)?;
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
    let result = match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("zip") => extract_zip(path, output),
        Some("7z") => extract_7z(path, output),
        Some("rar") => extract_rar(path, output),
        _ => bail!("unsupported archive"),
    };

    result.with_context(|| format!("failed to extract {}", path.display()))
}
