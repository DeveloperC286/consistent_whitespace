use std::path::PathBuf;

use crate::lexical_analysis::{File, Files, Line, Token};
use crate::{ConsistencyMode, WhitespacePreference};

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
    mode: &ConsistencyMode,
) -> Option<ConsistentWhitespaceErrors> {
    match mode {
        ConsistencyMode::WithinFiles => evaluate_within_files(files, whitespace_preference),
        ConsistencyMode::AcrossFiles => evaluate_across_files(files, whitespace_preference),
    }
}

fn evaluate_within_files(
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

fn evaluate_across_files(
    files: Files,
    whitespace_preference: &WhitespacePreference,
) -> Option<ConsistentWhitespaceErrors> {
    // First, check if all files use the same whitespace type
    let mut file_formats: Vec<(PathBuf, Format)> = Vec::new();

    for file in &files {
        let file_format = get_file_format(file, whitespace_preference);
        if let Some(format) = file_format {
            file_formats.push((file.path.clone(), format));
        }
    }

    if file_formats.is_empty() {
        return None;
    }

    let first_format = file_formats[0].1.clone();

    let inconsistent_files: Vec<ConsistentWhitespaceError> = file_formats
        .into_iter()
        .filter_map(|(path, format)| {
            if format != first_format {
                // Create an error for this file
                let file = files.iter().find(|f| f.path == path).unwrap();
                let lines: Vec<LineState> = file.lines.iter().map(evaluate_line).collect();
                Some(ConsistentWhitespaceError { path, lines })
            } else {
                None
            }
        })
        .collect();

    if inconsistent_files.is_empty() {
        None
    } else {
        Some(ConsistentWhitespaceErrors {
            errors: inconsistent_files,
        })
    }
}

fn get_file_format(file: &File, whitespace_preference: &WhitespacePreference) -> Option<Format> {
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

    // If file has mixed indentation, it's inconsistent regardless of preference
    if mixed > 0 {
        return Some(Format::Mixed);
    }

    match whitespace_preference {
        WhitespacePreference::Tabs => {
            if spaces > 0 {
                Some(Format::Spaces)
            } else if tabs > 0 {
                Some(Format::Tabs)
            } else {
                None
            }
        }
        WhitespacePreference::Spaces => {
            if tabs > 0 {
                Some(Format::Tabs)
            } else if spaces > 0 {
                Some(Format::Spaces)
            } else {
                None
            }
        }
        WhitespacePreference::Either => {
            if spaces > 0 && tabs == 0 {
                Some(Format::Spaces)
            } else if tabs > 0 && spaces == 0 {
                Some(Format::Tabs)
            } else if spaces == 0 && tabs == 0 {
                None
            } else {
                Some(Format::Mixed)
            }
        }
    }
}

pub fn evaluate_file(
    file: &File,
    whitespace_preference: &WhitespacePreference,
) -> Option<ConsistentWhitespaceError> {
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

#[derive(PartialEq, Debug, Clone)]
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
