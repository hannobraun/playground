use std::{fs::File, io::Read, path::Path};

use anyhow::bail;
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

        use SpecTestOutcome::*;

        let expected_outcome = {
            let Some(file_name) = path.file_name() else {
                unreachable!(
                    "Path is not a directory, but has no file name either."
                );
            };
            let Some(file_name) = file_name.to_str() else {
                bail!("Can't represent test file name as UTF-8.");
            };

            let Some((file_name_without_extension, _)) =
                file_name.rsplit_once(".")
            else {
                bail!("Expecting test file name to have an extension.");
            };
            let Some((_, pass_or_fail)) =
                file_name_without_extension.rsplit_once(".")
            else {
                panic!(
                    "Expecting test file name to have a pass/fail specifier."
                );
            };

            match pass_or_fail {
                "pass" => Pass,
                "fail" => Fail,

                unexpected => {
                    bail!("Unexpected pass/fail specifier (`{unexpected}`).");
                }
            }
        };

        match (evaluate(&code), expected_outcome) {
            (Ok(()), Pass) | (Err(_), Fail) => {
                print!("{}", "PASS".bold().with(Color::DarkGreen))
            }
            (Ok(()), Fail) | (Err(_), Pass) => {
                print!("{}", "FAIL".bold().with(Color::DarkRed))
            }
        }

        let path = path.strip_prefix(spec_dir)?;
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

enum SpecTestOutcome {
    Pass,
    Fail,
}
