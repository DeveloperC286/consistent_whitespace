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
    let mut classified_files: Vec<(PathBuf, Format, Vec<LineState>)> = Vec::new();

    for file in &files {
        let lines: Vec<LineState> = file.lines.iter().map(evaluate_line).collect();
        if let Some(format) = classify_lines(&lines, whitespace_preference) {
            classified_files.push((file.path.clone(), format, lines));
        }
    }

    if classified_files.is_empty() {
        return None;
    }

    // Determine the expected format based on the whitespace preference.
    // For Either, use the first non-mixed format seen; Mixed files are always errors.
    let expected_format: Option<Format> = match whitespace_preference {
        WhitespacePreference::Tabs => Some(Format::Tabs),
        WhitespacePreference::Spaces => Some(Format::Spaces),
        WhitespacePreference::Either => classified_files
            .iter()
            .find(|(_, f, _)| *f != Format::Mixed)
            .map(|(_, f, _)| f.clone()),
    };

    let inconsistent_files: Vec<ConsistentWhitespaceError> = classified_files
        .into_iter()
        .filter_map(|(path, format, lines)| {
            if is_inconsistent(&format, expected_format.as_ref()) {
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

/// Classifies a file's indentation, described by its per-line [`LineState`]s,
/// into a single [`Format`] relative to the given whitespace preference.
///
/// Returns `None` when the file has no indentation to judge. `Some(Format::Mixed)`
/// signals inconsistency: either individual lines mix spaces and tabs, or the
/// file combines space-indented and tab-indented lines under `Either`.
fn classify_lines(
    lines: &[LineState],
    whitespace_preference: &WhitespacePreference,
) -> Option<Format> {
    let (spaces, tabs, mixed) = count_line_formats(lines);

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

    let format = classify_lines(&lines, whitespace_preference)?;

    // Within a single file there is no cross-file expectation to satisfy, so
    // only Mixed files are errors under Either; Tabs/Spaces expect that format.
    let expected_format: Option<Format> = match whitespace_preference {
        WhitespacePreference::Tabs => Some(Format::Tabs),
        WhitespacePreference::Spaces => Some(Format::Spaces),
        WhitespacePreference::Either => None,
    };

    if is_inconsistent(&format, expected_format.as_ref()) {
        Some(ConsistentWhitespaceError {
            path: file.path.clone(),
            lines,
        })
    } else {
        None
    }
}

/// Counts how many lines are space-indented, tab-indented, and mixed. Lines with
/// no indentation ([`Format::None`]) are ignored.
fn count_line_formats(lines: &[LineState]) -> (usize, usize, usize) {
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

/// A classified file is inconsistent if it mixes whitespace within itself, or if
/// an expected format is known and the file does not match it.
fn is_inconsistent(format: &Format, expected: Option<&Format>) -> bool {
    *format == Format::Mixed || expected.is_some_and(|expected| format != expected)
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
