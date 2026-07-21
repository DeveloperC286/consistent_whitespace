use crate::evaluator::{ConsistentWhitespaceError, ConsistentWhitespaceErrors, Format};
use crate::ConsistencyMode;

pub fn report(errors: &ConsistentWhitespaceErrors, mode: &ConsistencyMode) {
    match mode {
        ConsistencyMode::WithinFiles => report_within_files(errors),
        ConsistencyMode::AcrossFiles => report_across_files(errors),
    }
}

fn report_within_files(errors: &ConsistentWhitespaceErrors) {
    for error in &errors.errors {
        report_error(error, "Inconsistent Formatting");
    }
}

fn report_across_files(errors: &ConsistentWhitespaceErrors) {
    for error in &errors.errors {
        report_error(error, "Uses different whitespace than other files");
    }
}

fn report_error(error: &ConsistentWhitespaceError, message: &str) {
    println!("::group::{}", error.path.display());

    // https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-commands#setting-an-error-message
    println!("::error file={}::{}", error.path.display(), message);

    for line in &error.lines {
        let format = match line.format {
            Format::Spaces => "Spaces",
            Format::Tabs => "Tabs",
            Format::Mixed => "Mixed",
            Format::None => "None",
        };

        println!(
            "::error file={},line={}::{}",
            error.path.display(),
            line.line_number,
            format
        );
    }

    println!("::endgroup::");
}
