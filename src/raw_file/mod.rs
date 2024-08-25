use std::path::{Path, PathBuf};

use anyhow::Result;

pub type RawLine = String;
pub type RawLines = Vec<RawLine>;
pub type RawFiles = Vec<RawFile>;

pub struct RawFile {
    pub lines: RawLines,
}

pub fn get_raw_files_in_directory(directory: PathBuf) -> Result<RawFiles> {
    let mut files = Vec::new();

    let entries = std::fs::read_dir(directory)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(get_raw_file(&path)?);
        } else {
            files.extend(get_raw_files_in_directory(path)?);
        }
    }

    Ok(files)
}

fn get_raw_file(path: &Path) -> Result<RawFile> {
    let lines = std::fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect();
    Ok(RawFile { lines })
}
