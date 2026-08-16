use std::process::Command;

use anyhow::bail;

pub fn run(description: &str, command: &str) -> anyhow::Result<()> {
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
