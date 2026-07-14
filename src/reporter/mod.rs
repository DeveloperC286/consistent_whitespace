mod github;
mod pretty;

use crate::evaluator::ConsistentWhitespaceErrors;
use crate::output::{Output, ResolvedOutput};
use crate::ConsistencyMode;

pub fn report(errors: &ConsistentWhitespaceErrors, mode: &ConsistencyMode, output: &Output) {
    match output.resolve() {
        ResolvedOutput::Quiet => {}
        ResolvedOutput::Pretty => pretty::report(errors, mode),
        ResolvedOutput::GitHub => github::report(errors, mode),
    }
}
