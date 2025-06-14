use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ignore::Walk;

pub type RawLines = Vec<RawLine>;
pub type RawFiles = Vec<RawFile>;

pub struct RawFile {
    pub path: PathBuf,
    pub lines: RawLines,
}

pub struct RawLine {
    pub line: String,
    pub line_number: usize,
}

pub fn get_raw_files(paths: &[PathBuf]) -> Result<RawFiles> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            files.push(get_raw_file(path)?);
        } else {
            files.extend(get_raw_files_in_directory(path)?);
        }
    }
    Ok(files)
}

fn get_raw_files_in_directory(directory: &Path) -> Result<RawFiles> {
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
    let lines = std::fs::read_to_string(path)
        .context(format!("Failed to read the file {}", path.display()))?
        .lines()
        .enumerate()
        .map(|(line_number, line)| RawLine {
            line: line.to_string(),
            line_number: line_number + 1,
        })
        .collect();
    Ok(RawFile {
        path: path.to_path_buf(),
        lines,
    })
}
