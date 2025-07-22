use std::{fs, path::Path, process::Command};

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

    let output = Path::new("output");

    fs::create_dir_all(output)?;
    fs::copy(
        "target/wasm32v1-none/release/wasm_reference.wasm",
        output.join("reference.wasm"),
    )?;

    Ok(())
}
