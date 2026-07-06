use std::path::PathBuf;

use crate::lexical_analysis::{File, Files, Line, Token};
use crate::WhitespacePreference;

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

pub fn evaluate(
    files: Files,
    whitespace_preference: &WhitespacePreference,
) -> Option<ConsistentWhitespaceErrors> {
    let errors: Vec<ConsistentWhitespaceError> = files
        .into_iter()
        .filter_map(|file| evaluate_file(&file, whitespace_preference))
        .collect();

    if errors.is_empty() {
        None
    } else {
        Some(ConsistentWhitespaceErrors { errors })
    }
}

pub fn evaluate_file(
    file: &File,
    whitespace_preference: &WhitespacePreference,
) -> Option<ConsistentWhitespaceError> {
    let lines: Vec<LineState> = file.lines.iter().map(evaluate_line).collect();

    let (spaces, tabs, mixed) = lines
        .iter()
        .fold((0usize, 0usize, 0usize), |(s, t, m), line| {
            match line.format {
                Format::Spaces => (s + 1, t, m),
                Format::Tabs => (s, t + 1, m),
                Format::Mixed => (s, t, m + 1),
                Format::None => (s, t, m),
            }
        });

    match whitespace_preference {
        WhitespacePreference::Tabs => {
            if spaces > 0 || mixed > 0 {
                return Some(ConsistentWhitespaceError {
                    path: file.path.clone(),
                    lines,
                });
            }
        }
        WhitespacePreference::Spaces => {
            if tabs > 0 || mixed > 0 {
                return Some(ConsistentWhitespaceError {
                    path: file.path.clone(),
                    lines,
                });
            }
        }
        WhitespacePreference::Either => {}
    };

    match (spaces, tabs, mixed) {
        // All lines are spaces or all lines are tabs - consistent
        (_, 0, 0) | (0, _, 0) => None,
        // Mixed indentation
        _ => Some(ConsistentWhitespaceError {
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
