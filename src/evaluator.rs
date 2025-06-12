use crate::lexical_analysis::{File, Files, Line, Token};

#[derive(PartialEq)]
pub enum State {
    Consistent,
    Inconsistent,
}

pub fn evaluate(files: Files) -> State {
    for file in files {
        let state = evaluate_file(&file);

        if state == State::Inconsistent {
            warn!("File {} is inconsistent.", file.path.display());
            return State::Inconsistent;
        }
    }

    State::Consistent
}

pub fn evaluate_file(file: &File) -> State {
    let formats: Vec<Format> = file.lines.iter().map(evaluate_line).collect();
    let spaces = formats.iter().filter(|&e| *e == Format::Spaces).count();
    let tabs = formats.iter().filter(|&e| *e == Format::Tabs).count();
    let mixed = formats.iter().filter(|&e| *e == Format::Mixed).count();

    match (spaces, tabs, mixed) {
        // All lines are spaces.
        (_, 0, 0) => State::Consistent,
        // All lines are tabs.
        (0, _, 0) => State::Consistent,
        (_, _, _) => State::Inconsistent,
    }
}

#[derive(PartialEq)]
pub enum Format {
    Spaces,
    Tabs,
    Mixed,
}

pub fn evaluate_line(line: &Line) -> Format {
    let spaces = line.iter().filter(|&e| *e == Token::Space).count();
    let tabs = line.iter().filter(|&e| *e == Token::Tab).count();

    match (spaces, tabs) {
        (0, _) => Format::Tabs,
        (_, 0) => Format::Spaces,
        (_, _) => Format::Mixed,
    }
}
