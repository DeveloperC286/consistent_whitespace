use std::path::PathBuf;

use super::*;
use crate::evaluator::LineState;

fn build_errors(files: &[(&str, &[(usize, Format)])]) -> ConsistentWhitespaceErrors {
    let errors = files
        .iter()
        .map(|(path, lines)| ConsistentWhitespaceError {
            path: PathBuf::from(path),
            lines: lines
                .iter()
                .map(|(line_number, format)| LineState {
                    line_number: *line_number,
                    format: format.clone(),
                })
                .collect(),
        })
        .collect();

    ConsistentWhitespaceErrors { errors }
}

#[test]
fn within_files_mixed_file() {
    // Given
    let errors = build_errors(&[(
        "src/main.rs",
        &[(1, Format::None), (2, Format::Spaces), (3, Format::Tabs)],
    )]);

    // When
    let output = report(&errors, &ConsistencyMode::WithinFiles);

    // Then
    insta::assert_snapshot!(output);
}

#[test]
fn across_files_single_file() {
    // Given
    let errors = build_errors(&[(
        "src/tabs.txt",
        &[(1, Format::None), (2, Format::Tabs), (3, Format::Tabs)],
    )]);

    // When
    let output = report(&errors, &ConsistencyMode::AcrossFiles);

    // Then
    insta::assert_snapshot!(output);
}

#[test]
fn across_files_multiple_files() {
    // Given
    let errors = build_errors(&[
        ("src/tabs.txt", &[(1, Format::Tabs), (2, Format::Tabs)]),
        ("src/mixed.txt", &[(1, Format::Spaces), (2, Format::Mixed)]),
    ]);

    // When
    let output = report(&errors, &ConsistencyMode::AcrossFiles);

    // Then
    insta::assert_snapshot!(output);
}
