use crate::lexical_analysis::{Files, Token};

pub enum State {
    Consistent,
    Inconsistent,
}

pub fn evaluate(files: Files) -> State {
    for file in files {
        for line in file.lines {
            let spaces = line.iter().filter(|&e| *e == Token::Space).count();
            let tabs = line.iter().filter(|&e| *e == Token::Tab).count();

            match (spaces, tabs) {
                (0, _) => {}
                (_, 0) => {}
                (_, _) => {
                    return State::Inconsistent;
                }
            }
        }
    }

    State::Consistent
}
