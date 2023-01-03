use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::*;

pub type RawFiles = Vec<RawFile>;

pub struct RawFile {
    pub filename: Filename,
    pub lines: Lines,
}

pub fn get_raw_files(
    current_working_directory: &str,
) -> Result<RawFiles, Box<dyn std::error::Error>> {
    get_raw_files_in_directory(current_working_directory)
}

fn get_raw_files_in_directory(directory: &str) -> Result<RawFiles, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    let entries = std::fs::read_dir(directory)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path.to_str().unwrap();

        if path.is_file() {
            files.push(get_raw_file(name)?);
        } else {
            files.extend(get_raw_files_in_directory(name)?);
        }
    }

    Ok(files)
}

fn get_raw_file(filename: &str) -> Result<RawFile, Box<dyn std::error::Error>> {
    let buffered_lines = read_lines(filename)?;
    let mut lines = vec![];

    for line in buffered_lines.flatten() {
        lines.push(line.to_string());
    }

    Ok(RawFile {
        filename: filename.to_string(),
        lines,
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
