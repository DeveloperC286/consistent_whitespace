#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use thiserror::Error;

mod consistent_whitespace_checker;
mod model;

#[derive(Error, Debug)]
enum ConsistentWhitespaceError {
    #[error("The current working directory's can not be expressed as a String.")]
    NoWorkingDirectoryString,
}

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));

    match get_current_working_directory() {
        Ok(current_working_directory) => {
            match crate::model::raw_file::get_raw_files(&current_working_directory) {
                Ok(raw_files) => {
                    if !crate::consistent_whitespace_checker::does_all_files_have_consistent_whitespace(raw_files) {
std::process::exit(ERROR_EXIT_CODE);
                    }
                }
                Err(error) => {
                    error!("{error:?}");
                    std::process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        Err(error) => {
            error!("{error:?}");
            std::process::exit(ERROR_EXIT_CODE);
        }
    }
}

fn get_current_working_directory() -> Result<String, Box<dyn std::error::Error>> {
    match std::env::current_dir()?.to_str() {
        Some(current_working_directory) => Ok(current_working_directory.to_string()),
        None => Err(Box::new(
            ConsistentWhitespaceError::NoWorkingDirectoryString,
        )),
    }
}
