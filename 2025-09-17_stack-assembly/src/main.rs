use std::{fs::File, io::Read};

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
            Ok(()) => print!("PASS"),
            Err(()) => print!("FAIL"),
        }
        println!(" {path}", path = entry.path().display());
    }

    Ok(())
}

fn run_spec_script(code: &str) -> Result<(), ()> {
    let mut stack = Vec::new();

    for token in code.split_whitespace() {
        match token {
            "=" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

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
                let a = stack.pop().unwrap();

                if a == 0 {
                    return Err(());
                }
            }
            "1" => {
                stack.push(1);
            }
            "2" => {
                stack.push(2);
            }
            token => {
                panic!("Unexpected token: {token}");
            }
        }
    }

    Ok(())
}
