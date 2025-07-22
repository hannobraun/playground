use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    process::Command,
};

use wasmtime::{Engine, Instance, Module, Store};

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
    let reference_module = output.join("reference.wasm");

    fs::create_dir_all(output)?;
    fs::copy(
        "target/wasm32v1-none/release/wasm_reference.wasm",
        &reference_module,
    )?;

    let mut module = Vec::new();
    File::open(reference_module)?.read_to_end(&mut module)?;

    let engine = Engine::default();
    let module = Module::new(&engine, module)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let start = instance.get_typed_func::<(), i32>(&mut store, "start")?;
    let result = start.call(&mut store, ())?;

    println!();
    if result != 42 {
        anyhow::bail!("Unexpected result: {result}");
    }

    Ok(())
}
