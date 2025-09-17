use std::{fs::File, io::Read};

use crossterm::style::{Color, Stylize};
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    for entry in WalkDir::new("spec") {
        let entry = entry?;

        if entry.path().is_dir() {
            continue;
        }

        let mut code = String::new();
        File::open(entry.path())?.read_to_string(&mut code)?;

        match run_spec_script(&code) {
            Ok(()) => print!("{}", "PASS".bold().with(Color::DarkGreen)),
            Err(()) => print!("{}", "FAIL".bold().with(Color::DarkRed)),
        }
        println!(" {path}", path = entry.path().display());
    }

    Ok(())
}

fn run_spec_script(code: &str) -> Result<(), ()> {
    let mut stack = Stack { inner: Vec::new() };

    for token in code.split_whitespace() {
        match token {
            "=" => {
                let b = stack.inner.pop().unwrap();
                let a = stack.inner.pop().unwrap();

                match a == b {
                    false => {
                        stack.inner.push(0);
                    }
                    true => {
                        stack.inner.push(1);
                    }
                }
            }
            "assert" => {
                let a = stack.inner.pop().unwrap();

                if a == 0 {
                    return Err(());
                }
            }
            "1" => {
                stack.inner.push(1);
            }
            "2" => {
                stack.inner.push(2);
            }
            _ => {
                return Err(());
            }
        }
    }

    Ok(())
}

pub struct Stack {
    pub inner: Vec<i32>,
}
