use std::process::Command;

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    run("Check formatting", "cargo fmt --check")?;
    // Without `--all-targets`, Clippy will not check code gated by
    // `#[cfg(test)]`.
    run("Run Clippy", "cargo clippy --all-targets")?;
    run("Run test suite", "cargo test")?;
    run("Run mutation testing", "cargo mutants")?;
    run("Build documentation", "cargo doc")?;

    Ok(())
}

fn run(description: &str, command: &str) -> anyhow::Result<()> {
    println!();
    println!();
    println!("=== {description}");
    println!();

    let status = Command::new("sh").args(["-c", command]).status()?;

    if !status.success() {
        bail!("Command `{command}` failed with status `{status:?}`");
    }

    Ok(())
}
