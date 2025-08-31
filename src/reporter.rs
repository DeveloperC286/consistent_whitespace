use crate::evaluator::{ConsistentWhitespaceErrors, Format};

pub fn report(errors: &ConsistentWhitespaceErrors) {
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
