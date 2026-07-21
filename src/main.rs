use anyhow::Result;
use clap::Parser;
use log::{debug, error, info};

mod evaluator;
mod lexical_analysis;
mod raw_file;
mod reporter;

const ERROR_EXIT_CODE: i32 = 1;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum WhitespacePreference {
    /// Only accept tabs as whitespace.
    Tabs,
    /// Only accept spaces as whitespace.
    Spaces,
    /// Accept either tabs or spaces as long as they are consistent.
    Either,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ConsistencyMode {
    /// Check consistency within each individual file (legacy behavior).
    WithinFiles,
    /// Check consistency across all files (default behavior).
    AcrossFiles,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Files or directories to check. If none are provided, the current directory is used.
    #[arg(value_name = "PATH", value_parser, num_args = 0.., default_value = ".")]
    paths: Vec<std::path::PathBuf>,

    /// The only whitespacing that can be used. If not specified, either tabs or spaces are acceptable as long as they are consistent.
    #[arg(long, value_enum, default_value = "either")]
    whitespace: WhitespacePreference,

    /// The mode for checking whitespace consistency.
    #[arg(long, value_enum, default_value = "across-files")]
    mode: ConsistencyMode,

    #[arg(
        long,
        help = "Enable verbose output, respects RUST_LOG environment variable if set."
    )]
    verbose: bool,
}

fn main() {
    let arguments = Arguments::parse();

    // Set up logging. Log level precedence:
    // - RUST_LOG, if set.
    // - info, if --verbose is passed.
    let mut logger = pretty_env_logger::formatted_builder();
    match std::env::var("RUST_LOG") {
        Ok(rust_log) => {
            logger.parse_filters(&rust_log);
        }
        Err(_) if arguments.verbose => {
            logger.filter_level(log::LevelFilter::Info);
        }
        Err(_) => {}
    }
    logger.init();

    info!("Version {}.", env!("CARGO_PKG_VERSION"));
    debug!("The command line arguments provided are {arguments:?}.");

    match run(arguments) {
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
        Err(err) => {
            error!("{err:?}");
            std::process::exit(ERROR_EXIT_CODE);
        }
    }
}

fn run(arguments: Arguments) -> Result<i32> {
    let raw_files = raw_file::get_raw_files(&arguments.paths)?;
    let files = lexical_analysis::parse(raw_files);
    if let Some(errors) = evaluator::evaluate(files, &arguments.whitespace, &arguments.mode) {
        reporter::report(&errors, &arguments.mode);
        Ok(1)
    } else {
        Ok(0)
    }
}
