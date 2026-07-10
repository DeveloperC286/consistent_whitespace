use crate::evaluator::{ConsistentWhitespaceErrors, Format};
use crate::ConsistencyMode;

pub fn report(errors: &ConsistentWhitespaceErrors, mode: &ConsistencyMode) {
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
}

fn report_across_files(errors: &ConsistentWhitespaceErrors) {
    println!("Files have inconsistent whitespace across the codebase:");
    for error in &errors.errors {
        println!(
            "  {}: Uses different whitespace than other files",
            error.path.display()
        );
    }
    println!();
    println!("All files must use the same whitespace type (spaces or tabs) for consistency.");
}
