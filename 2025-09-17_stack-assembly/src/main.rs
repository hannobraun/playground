use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    for entry in WalkDir::new("spec") {
        let entry = entry?;

        if entry.path().is_dir() {
            continue;
        }

        dbg!(entry);
    }

    Ok(())
}
