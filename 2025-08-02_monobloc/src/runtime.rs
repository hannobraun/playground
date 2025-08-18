use anyhow::anyhow;
use wasmtime::{Engine, Instance, Module, Store};

pub fn evaluate_root(code: &[u8]) -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::new(&engine, code)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let root = instance
        .get_func(&mut store, "root")
        .ok_or_else(|| anyhow!("Could not find root function."))?;
    root.call(&mut store, &[], &mut [])?;

    Ok(())
}
