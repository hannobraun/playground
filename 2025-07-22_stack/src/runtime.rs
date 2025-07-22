use wasmtime::{Engine, Instance, Module, Store};

pub fn execute(code: &[u8]) -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::new(&engine, code)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let start = instance.get_typed_func::<(), i32>(&mut store, "start")?;
    let result = start.call(&mut store, ())?;

    println!();
    if result == 42 {
        println!("✅ `start` returned {result}");
    } else {
        anyhow::bail!("Unexpected result: {result}");
    }

    Ok(())
}
