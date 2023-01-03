use regex::Regex;

use crate::model::raw_file::RawFile;

pub(crate) fn does_all_files_have_consistent_whitespace(raw_files: Vec<RawFile>) -> bool {
    raw_files
        .into_iter()
        .map(does_file_have_consistent_whitespace)
        .all(|result| result)
}

pub(crate) fn does_file_have_consistent_whitespace(raw_file: RawFile) -> bool {
    let pattern = r"^([ ]+\t|[\t]+[ ])";
    let re = Regex::new(pattern).unwrap();

    for line in raw_file.lines {
        if re.is_match(&line) {
            error!(
                "The file {0:?} has inconsistent whitespace.",
                raw_file.filename
            );
            return false;
        }
    }

    true
}
