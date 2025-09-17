use std::path::Path;

use walkdir::WalkDir;

use crate::spec::run_test;

mod evaluate;
mod spec;
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
