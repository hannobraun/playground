use crate::util::run;

pub fn build() -> anyhow::Result<()> {
    run("Check formatting", "cargo fmt --check")?;
    // Without `--all-targets`, Clippy will not check code gated by
    // `#[cfg(test)]`.
    run("Run Clippy", "cargo clippy --all-targets")?;
    run("Run test suite", "cargo test")?;
    run("Run mutation testing", "cargo mutants")?;
    run("Build documentation", "cargo doc")?;

    Ok(())
}
