use std::path::Path;

use walkdir::WalkDir;

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

        spec::run_test(spec_dir, path)?;
    }

    Ok(())
}
