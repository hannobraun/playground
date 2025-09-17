use std::{fs::File, io::Read, path::Path};

use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    for entry in WalkDir::new("spec") {
        let entry = entry?;

        if entry.path().is_dir() {
            continue;
        }

        use SpecScriptOutcome::*;
        match run_spec_script(entry.path())? {
            Pass => print!("PASS"),
            Fail => print!("FAIL"),
        }
        println!(" {path}", path = entry.path().display());
    }

    Ok(())
}

fn run_spec_script(path: &Path) -> anyhow::Result<SpecScriptOutcome> {
    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;

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
                    return Ok(SpecScriptOutcome::Fail);
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

    Ok(SpecScriptOutcome::Pass)
}

enum SpecScriptOutcome {
    Pass,
    Fail,
}
