use std::{fs::File, io::Read, path::Path};

use crossterm::style::{Color, Stylize};
use walkdir::WalkDir;

use crate::stack::Stack;

mod stack;

fn main() -> anyhow::Result<()> {
    let spec_dir = Path::new("spec");

    for entry in WalkDir::new(spec_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let mut code = String::new();
        File::open(path)?.read_to_string(&mut code)?;

        match evaluate(&code) {
            Ok(()) => print!("{}", "PASS".bold().with(Color::DarkGreen)),
            Err(_) => print!("{}", "FAIL".bold().with(Color::DarkRed)),
        }
        println!(" {path}", path = path.display());
    }

    Ok(())
}

fn evaluate(code: &str) -> Result<(), EvaluateError> {
    let mut stack = Stack::new();

    for token in code.split_whitespace() {
        match token {
            "=" => {
                let b = stack.pop();
                let a = stack.pop();

                match a == b {
                    false => {
                        stack.push(0);
                    }
                    true => {
                        stack.push(1);
                    }
                }
            }
            "assert" => {
                let a = stack.pop();

                if a == 0 {
                    return Err(EvaluateError::Other);
                }
            }
            "1" => {
                stack.push(1);
            }
            "2" => {
                stack.push(2);
            }
            _ => {
                return Err(EvaluateError::Other);
            }
        }
    }

    Ok(())
}

enum EvaluateError {
    Other,
}
