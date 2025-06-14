use std::path::PathBuf;

use crate::raw_file::{RawFiles, RawLine};

pub type Lines = Vec<Line>;
pub type Files = Vec<File>;

pub struct File {
    pub path: PathBuf,
    pub lines: Lines,
}

pub struct Line {
    pub tokens: Vec<Token>,
    pub line_number: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Tab,
    Space,
}

pub fn parse(raw_files: RawFiles) -> Files {
    raw_files
        .into_iter()
        .map(|raw_file| File {
            path: raw_file.path,
            lines: raw_file.lines.into_iter().map(parse_line).collect(),
        })
        .collect()
}

fn parse_line(raw_line: RawLine) -> Line {
    let mut tokens = vec![];

    for character in raw_line.line.chars() {
        match character {
            ' ' => {
                tokens.push(Token::Space);
            }
            '\t' => {
                tokens.push(Token::Tab);
            }
            _ => {
                break;
            }
        }
    }

    Line {
        tokens,
        line_number: raw_line.line_number,
    }
}
