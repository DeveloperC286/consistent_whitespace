#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use anyhow::Result;
use clap::Parser;

mod evaluator;
mod lexical_analysis;
mod raw_file;
mod reporter;

const ERROR_EXIT_CODE: i32 = 1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Files or directories to check. If none are provided, the current directory is used.
    #[arg(value_name = "PATH", value_parser, num_args = 0.., default_value = ".")]
    paths: Vec<std::path::PathBuf>,
}

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = Arguments::parse();
    debug!("The command line arguments provided are {:?}.", arguments);

    match run(arguments) {
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
        Err(err) => {
            error!("{:?}", err);
            std::process::exit(ERROR_EXIT_CODE);
        }
    }
}

fn run(arguments: Arguments) -> Result<i32> {
    let raw_files = raw_file::get_raw_files(&arguments.paths)?;
    let files = lexical_analysis::parse(raw_files);
    if let Some(errors) = evaluator::evaluate(files) {
        reporter::report(&errors);
        Ok(1)
    } else {
        Ok(0)
    }
}
