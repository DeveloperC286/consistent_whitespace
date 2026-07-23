use std::fmt::Write;

use crate::ConsistencyMode;
use crate::evaluator::{ConsistentWhitespaceError, ConsistentWhitespaceErrors, Format};

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
mod tests;
