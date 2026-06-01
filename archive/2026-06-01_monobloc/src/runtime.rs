use std::iter;

use anyhow::anyhow;
use wasmtime::{Engine, Instance, Module, Store, Val};

use crate::compiler::ir::Package;

pub fn evaluate_root(
    code: &[u8],
    package: &Package,
) -> anyhow::Result<Vec<i32>> {
    let engine = Engine::default();
    let module = Module::new(&engine, code)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let func = instance
        .get_func(&mut store, "root")
        .ok_or_else(|| anyhow!("Could not find root function."))?;

    let signature = &package.signatures[package.root().signature];
    let mut results = iter::repeat_n(Val::I32(0), signature.outputs.len())
        .collect::<Vec<_>>();
    func.call(&mut store, &[], &mut results)?;

    let output = results
        .into_iter()
        .map(|val| {
            if let Val::I32(value) = val {
                value
            } else {
                panic!("Invalid result from root function: `{val:?}`");
            }
        })
        .collect();

    Ok(output)
}
