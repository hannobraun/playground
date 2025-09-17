use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    for entry in WalkDir::new("spec") {
        let entry = entry?;
        dbg!(entry);
    }

    Ok(())
}
