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

pub fn get_raw_files(search: &[PathBuf]) -> Result<RawFiles> {
    let files = get_files_paths(search)?;
    get_raw_files_from_paths(files)
}

fn get_files_paths(search: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in search {
        files.extend(get_file_paths(path.as_path())?);
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn get_file_paths(search: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for entry in Walk::new(search) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            paths.push(path.to_path_buf());
        }
    }

    Ok(paths)
}

fn get_raw_files_from_paths(paths: Vec<PathBuf>) -> Result<RawFiles> {
    let mut files = Vec::new();
    for path in paths {
        files.push(get_raw_file(&path)?);
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
