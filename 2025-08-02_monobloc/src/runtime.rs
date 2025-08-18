use wasmtime::{Engine, Instance, Module, Store};

pub fn evaluate_root(code: &[u8]) -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::new(&engine, code)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let root = instance.get_typed_func::<(), ()>(&mut store, "root")?;
    let () = root.call(&mut store, ())?;

    Ok(())
}
