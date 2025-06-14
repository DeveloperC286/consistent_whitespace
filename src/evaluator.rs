use std::path::PathBuf;

use crate::lexical_analysis::{File, Files, Line, Token};

pub struct ConsistentWhitespaceErrors {
    pub errors: Vec<ConsistentWhitespaceError>,
}

pub struct ConsistentWhitespaceError {
    pub path: PathBuf,
    pub lines: Vec<LineState>,
}

pub struct LineState {
    pub line_number: usize,
    pub format: Format,
}

pub fn evaluate(files: Files) -> Option<ConsistentWhitespaceErrors> {
    let errors: Vec<ConsistentWhitespaceError> = files
        .into_iter()
        .filter_map(|file| evaluate_file(&file))
        .collect();

    if errors.is_empty() {
        None
    } else {
        Some(ConsistentWhitespaceErrors { errors })
    }
}

pub fn evaluate_file(file: &File) -> Option<ConsistentWhitespaceError> {
    let lines: Vec<LineState> = file.lines.iter().map(evaluate_line).collect();

    let spaces = lines
        .iter()
        .filter(|&line| line.format == Format::Spaces)
        .count();
    let tabs = lines
        .iter()
        .filter(|&line| line.format == Format::Tabs)
        .count();
    let mixed = lines
        .iter()
        .filter(|&line| line.format == Format::Mixed)
        .count();
    let none = lines
        .iter()
        .filter(|&line| line.format == Format::None)
        .count();

    match (spaces, tabs, mixed, none) {
        // All lines are spaces or all lines are tabs - consistent
        (_, 0, 0, _) | (0, _, 0, _) => None,
        // Mixed indentation
        (_, _, _, _) => Some(ConsistentWhitespaceError {
            path: file.path.clone(),
            lines,
        }),
    }
}

#[derive(PartialEq)]
pub enum Format {
    Spaces,
    Tabs,
    Mixed,
    None,
}

pub fn evaluate_line(line: &Line) -> LineState {
    let spaces = line.tokens.iter().filter(|&e| *e == Token::Space).count();
    let tabs = line.tokens.iter().filter(|&e| *e == Token::Tab).count();

    let format = if line.tokens.is_empty() {
        Format::None
    } else {
        match (spaces, tabs) {
            (0, _) => Format::Tabs,
            (_, 0) => Format::Spaces,
            (_, _) => Format::Mixed,
        }
    };

    LineState {
        line_number: line.line_number,
        format,
    }
}
