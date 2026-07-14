mod github;
mod pretty;

use crate::ConsistencyMode;
use crate::evaluator::ConsistentWhitespaceErrors;
use crate::output::{Output, ResolvedOutput};

pub fn report(
    errors: &ConsistentWhitespaceErrors,
    mode: &ConsistencyMode,
    output: &Output,
) -> Option<String> {
    match output.resolve() {
        ResolvedOutput::Quiet => None,
        ResolvedOutput::Pretty => Some(pretty::report(errors, mode)),
        ResolvedOutput::GitHub => Some(github::report(errors, mode)),
    }
}

#[cfg(test)]
mod tests;
