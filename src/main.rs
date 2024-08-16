#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use anyhow::{anyhow, Result};

mod evaluator;
mod lexical_analysis;
mod raw_file;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));

    if let Err(err) = run() {
        error!("{:?}", err);
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run() -> Result<()> {
    let path = std::env::current_dir()?;
    let raw_files = raw_file::get_raw_files_in_directory(path)?;
    let files = lexical_analysis::parse(raw_files);
    match evaluator::evaluate(files) {
        evaluator::State::Consistent => Ok(()),
        evaluator::State::Inconsistent => Err(anyhow!("A file has inconsistent whitespace.")),
    }
}
