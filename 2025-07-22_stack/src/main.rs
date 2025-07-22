use std::process::Command;

fn main() -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .arg("build")
        .args(["--package", "wasm-reference"])
        .args(["--target", "wasm32v1-none"])
        .arg("--release")
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to build `wasm-reference`.");
    }

    Ok(())
}
