use crate::evaluator::{ConsistentWhitespaceErrors, Format};
use crate::ConsistencyMode;

pub(super) fn report(errors: &ConsistentWhitespaceErrors, mode: &ConsistencyMode) {
    match mode {
        ConsistencyMode::WithinFiles => report_within_files(errors),
        ConsistencyMode::AcrossFiles => report_across_files(errors),
    }
}

fn report_within_files(errors: &ConsistentWhitespaceErrors) {
    for error in &errors.errors {
        println!("::group::{}", error.path.display());

        // https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-commands#setting-an-error-message
        println!(
            "::error file={}::Inconsistent Formatting",
            error.path.display()
        );

        for line in &error.lines {
            println!(
                "::error file={},line={}::{}",
                error.path.display(),
                line.line_number,
                format_name(&line.format)
            );
        }

        println!("::endgroup::");
    }
}

fn report_across_files(errors: &ConsistentWhitespaceErrors) {
    for error in &errors.errors {
        println!(
            "::error file={}::Uses different whitespace than other files in the codebase.",
            error.path.display()
        );
    }
}

fn format_name(format: &Format) -> &'static str {
    match format {
        Format::Spaces => "Spaces",
        Format::Tabs => "Tabs",
        Format::Mixed => "Mixed",
        Format::None => "None",
    }
}
