use std::fmt::Write;

use crate::ConsistencyMode;
use crate::evaluator::{ConsistentWhitespaceErrors, Format};

const RED_BOLD: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";

pub(super) fn report(errors: &ConsistentWhitespaceErrors, mode: &ConsistencyMode) -> String {
    match mode {
        ConsistencyMode::WithinFiles => report_within_files(errors),
        ConsistencyMode::AcrossFiles => report_across_files(errors),
    }
}

fn report_within_files(errors: &ConsistentWhitespaceErrors) -> String {
    let mut output = String::new();

    for error in &errors.errors {
        let _ = writeln!(
            output,
            "{RED_BOLD}X{RESET} {} - Inconsistent Formatting",
            error.path.display()
        );

        for line in &error.lines {
            let _ = writeln!(
                output,
                "    line {}: {}",
                line.line_number,
                format_name(&line.format)
            );
        }
    }

    let total_linting_errors: usize = errors.errors.iter().map(|error| error.lines.len()).sum();
    let _ = writeln!(
        output,
        "{RED_BOLD}X{RESET} Found {total_linting_errors} separate linting errors across {} files.",
        errors.errors.len()
    );

    output
}

fn report_across_files(errors: &ConsistentWhitespaceErrors) -> String {
    let mut output = String::new();

    let _ = writeln!(
        output,
        "Files have inconsistent whitespace across the codebase:"
    );
    for error in &errors.errors {
        let _ = writeln!(
            output,
            "  {RED_BOLD}X{RESET} {}: Uses different whitespace than other files",
            error.path.display()
        );
    }
    let _ = writeln!(output);
    let _ = writeln!(
        output,
        "All files must use the same whitespace type (spaces or tabs) for consistency."
    );

    output
}

fn format_name(format: &Format) -> &'static str {
    match format {
        Format::Spaces => "Spaces",
        Format::Tabs => "Tabs",
        Format::Mixed => "Mixed",
        Format::None => "None",
    }
}
