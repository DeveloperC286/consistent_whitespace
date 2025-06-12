#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use anyhow::{anyhow, Result};
use clap::Parser;

mod evaluator;
mod lexical_analysis;
mod raw_file;

const ERROR_EXIT_CODE: i32 = 1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {}

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = Arguments::parse();
    debug!("The command line arguments provided are {:?}.", arguments);

    if let Err(err) = run() {
        error!("{:?}", err);
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run() -> Result<()> {
    let path = std::env::current_dir()?;
    let raw_files = raw_file::get_raw_files_in_directory(&path)?;
    let files = lexical_analysis::parse(raw_files);
    match evaluator::evaluate(files) {
        evaluator::State::Consistent => Ok(()),
        evaluator::State::Inconsistent => Err(anyhow!("A file has inconsistent whitespace.")),
    }
}
