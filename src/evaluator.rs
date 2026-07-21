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

    // Determine the expected format based on the whitespace preference.
    // For Either, use the first non-mixed format seen; Mixed files are always errors.
    let expected_format: Option<Format> = match whitespace_preference {
        WhitespacePreference::Tabs => Some(Format::Tabs),
        WhitespacePreference::Spaces => Some(Format::Spaces),
        WhitespacePreference::Either => file_formats
            .iter()
            .find(|(_, f)| *f != Format::Mixed)
            .map(|(_, f)| f.clone()),
    };

    let inconsistent_files: Vec<ConsistentWhitespaceError> = file_formats
        .into_iter()
        .filter_map(|(path, format)| {
            let is_inconsistent = format == Format::Mixed
                || expected_format
                    .as_ref()
                    .is_some_and(|expected| &format != expected);

            if is_inconsistent {
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

    let (spaces, tabs, mixed) = count_formats(&lines);

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

    let (spaces, tabs, mixed) = count_formats(&lines);

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

#[derive(PartialEq, Debug, Clone)]
pub enum Format {
    Spaces,
    Tabs,
    Mixed,
    None,
}

/// Counts how many lines fall into each indentation format, returning
/// `(spaces, tabs, mixed)`. Lines with `Format::None` are ignored.
fn count_formats(lines: &[LineState]) -> (usize, usize, usize) {
    lines
        .iter()
        .fold((0usize, 0usize, 0usize), |(s, t, m), line| {
            match line.format {
                Format::Spaces => (s + 1, t, m),
                Format::Tabs => (s, t + 1, m),
                Format::Mixed => (s, t, m + 1),
                Format::None => (s, t, m),
            }
        })
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
