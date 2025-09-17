use std::{fs::File, io::Read, path::Path};

use anyhow::bail;
use crossterm::style::{Color, Stylize};
use walkdir::WalkDir;

use crate::evaluate::evaluate;

mod evaluate;
mod stack;

fn main() -> anyhow::Result<()> {
    let spec_dir = Path::new("spec");

    for entry in WalkDir::new(spec_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        run_test(spec_dir, path)?;
    }

    Ok(())
}

pub fn run_test(spec_dir: &Path, test_file: &Path) -> anyhow::Result<()> {
    assert!(!test_file.is_dir());

    let mut code = String::new();
    File::open(test_file)?.read_to_string(&mut code)?;

    use SpecTestOutcome::*;

    let expected_outcome = {
        let Some(file_name) = test_file.file_name() else {
            unreachable!(
                "Path is not a directory, but has no file name either."
            );
        };
        let Some(file_name) = file_name.to_str() else {
            bail!("Can't represent test file name as UTF-8.");
        };

        let Some((file_name_without_extension, _)) = file_name.rsplit_once(".")
        else {
            bail!("Expecting test file name to have an extension.");
        };
        let Some((_, pass_or_fail)) =
            file_name_without_extension.rsplit_once(".")
        else {
            panic!("Expecting test file name to have a pass/fail specifier.");
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

    let path = test_file.strip_prefix(spec_dir)?;
    println!(" {path}", path = path.display());

    Ok(())
}

enum SpecTestOutcome {
    Pass,
    Fail,
}
