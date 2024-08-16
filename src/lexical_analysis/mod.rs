use crate::raw_file::RawFiles;

pub type Filename = String;
pub type Line = Vec<Token>;
pub type Lines = Vec<Line>;
pub type Files = Vec<File>;

pub struct File {
    pub filename: Filename,
    pub lines: Lines,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Tab,
    Space,
}

pub fn parse(raw_files: RawFiles) -> Files {
    raw_files
        .into_iter()
        .map(|raw_file| File {
            filename: raw_file.filename,
            lines: raw_file.lines.into_iter().map(parse_line).collect(),
        })
        .collect()
}

fn parse_line(raw_line: String) -> Line {
    let mut line = vec![];

    for character in raw_line.chars() {
        match character {
            ' ' => {
                line.push(Token::Space);
            }
            '\t' => {
                line.push(Token::Tab);
            }
            _ => {
                break;
            }
        }
    }

    line
}
