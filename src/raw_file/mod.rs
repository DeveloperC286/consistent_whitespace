use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use anyhow::Result;

pub type Filename = String;
pub type RawLine = String;
pub type RawLines = Vec<RawLine>;
pub type RawFiles = Vec<RawFile>;

pub struct RawFile {
    pub filename: Filename,
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
    let buffered_lines = read_lines(path)?;
    let mut lines = vec![];

    for line in buffered_lines.flatten() {
        lines.push(line.to_string());
    }

    Ok(RawFile {
        filename: path.to_str().unwrap().to_string(),
        lines,
    })
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
