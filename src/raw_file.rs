use std::path::{Path, PathBuf};

use anyhow::Result;
use ignore::Walk;

pub type RawLine = String;
pub type RawLines = Vec<RawLine>;
pub type RawFiles = Vec<RawFile>;

pub struct RawFile {
    pub path: PathBuf,
    pub lines: RawLines,
}

pub fn get_raw_files_in_directory(directory: &Path) -> Result<RawFiles> {
    let mut files = Vec::new();

    for entry in Walk::new(directory) {
        let entry = entry?;
        let path = entry.path();

        if path != directory {
            if path.is_file() {
                files.push(get_raw_file(path)?);
            } else {
                files.extend(get_raw_files_in_directory(path)?);
            }
        }
    }

    Ok(files)
}

fn get_raw_file(path: &Path) -> Result<RawFile> {
    trace!("Reading in the file {}.", path.display());
    let lines = std::fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect();
    Ok(RawFile {
        path: path.to_path_buf(),
        lines,
    })
}
