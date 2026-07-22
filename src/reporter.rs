use std::fmt::Write;

use crate::evaluator::{ConsistentWhitespaceError, ConsistentWhitespaceErrors, Format};
use crate::ConsistencyMode;

pub fn report(errors: &ConsistentWhitespaceErrors, mode: &ConsistencyMode) -> String {
    match mode {
        ConsistencyMode::WithinFiles => report_within_files(errors),
        ConsistencyMode::AcrossFiles => report_across_files(errors),
    }
}

fn report_within_files(errors: &ConsistentWhitespaceErrors) -> String {
    errors
        .errors
        .iter()
        .map(|error| report_error(error, "Inconsistent Formatting"))
        .collect()
}

fn report_across_files(errors: &ConsistentWhitespaceErrors) -> String {
    errors
        .errors
        .iter()
        .map(|error| report_error(error, "Uses different whitespace than other files"))
        .collect()
}

fn report_error(error: &ConsistentWhitespaceError, message: &str) -> String {
    let mut output = String::new();

    writeln!(output, "::group::{}", error.path.display()).unwrap();

    // https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-commands#setting-an-error-message
    writeln!(output, "::error file={}::{}", error.path.display(), message).unwrap();

    for line in &error.lines {
        let format = match line.format {
            Format::Spaces => "Spaces",
            Format::Tabs => "Tabs",
            Format::Mixed => "Mixed",
            Format::None => "None",
        };

        writeln!(
            output,
            "::error file={},line={}::{}",
            error.path.display(),
            line.line_number,
            format
        )
        .unwrap();
    }

    writeln!(output, "::endgroup::").unwrap();

    output
}

#[cfg(test)]
mod tests {
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
        let errors = build_errors(&[(
            "src/main.rs",
            &[
                (1, Format::None),
                (2, Format::Spaces),
                (3, Format::Tabs),
            ],
        )]);

        insta::assert_snapshot!(report(&errors, &ConsistencyMode::WithinFiles));
    }

    #[test]
    fn across_files_single_file() {
        let errors = build_errors(&[(
            "src/tabs.txt",
            &[(1, Format::None), (2, Format::Tabs), (3, Format::Tabs)],
        )]);

        insta::assert_snapshot!(report(&errors, &ConsistencyMode::AcrossFiles));
    }

    #[test]
    fn across_files_multiple_files() {
        let errors = build_errors(&[
            (
                "src/tabs.txt",
                &[(1, Format::Tabs), (2, Format::Tabs)],
            ),
            (
                "src/mixed.txt",
                &[(1, Format::Spaces), (2, Format::Mixed)],
            ),
        ]);

        insta::assert_snapshot!(report(&errors, &ConsistencyMode::AcrossFiles));
    }
}
