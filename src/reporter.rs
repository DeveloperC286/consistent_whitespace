use crate::evaluator::{ConsistentWhitespaceErrors, Format};

pub fn report(errors: &ConsistentWhitespaceErrors) {
    for error in &errors.errors {
        println!(
            "File: {} has inconsistent whitespace:",
            error.path.display()
        );
        for line in &error.lines {
            println!(
                "  Line {}: {}",
                line.line_number,
                match line.format {
                    Format::Spaces => "Spaces",
                    Format::Tabs => "Tabs",
                    Format::Mixed => "Mixed",
                    Format::None => "None",
                }
            );
        }
        println!();
    }
}
