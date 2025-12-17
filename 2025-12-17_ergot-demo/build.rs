use std::{env, fs::File, io::Write, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let memory_x = include_bytes!("memory.x");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    File::create(out_dir.join("memory.x"))?.write_all(memory_x)?;
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");

    Ok(())
}
