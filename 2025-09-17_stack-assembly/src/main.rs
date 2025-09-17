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

        for token in code.split_whitespace() {
            dbg!(token);
        }
    }

    Ok(())
}
