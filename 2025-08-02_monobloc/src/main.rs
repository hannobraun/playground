use std::{fs::File, io::Read};

use anyhow::Context;
use wasmtime::{Engine, Instance, Module, Store};

use crate::compiler::wasm;

mod compiler;

fn main() -> anyhow::Result<()> {
    let input_code = read_input_code("numbers.mbl")?;
    println!("{input_code}");

    let mut stack = Vec::<i32>::new();

    for identifier in input_code.split_whitespace() {
        if let Ok(value) = identifier.parse() {
            stack.push(value);
        } else {
            println!("Unknown identifier: `{identifier}`");
        }
    }

    let wasm_code = wasm::compile_module();
    run_wasm_module(&wasm_code)?;

    for (i, value) in stack.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }

        print!("{value}");
    }

    println!();

    Ok(())
}

fn read_input_code(path: &str) -> anyhow::Result<String> {
    let mut buf = String::new();

    File::open(path)
        .with_context(|| format!("Opening `{path}`"))?
        .read_to_string(&mut buf)
        .with_context(|| format!("Reading code from `{path}`"))?;

    Ok(buf)
}

fn run_wasm_module(code: &[u8]) -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::new(&engine, code)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let root = instance.get_typed_func::<(), ()>(&mut store, "root")?;
    let () = root.call(&mut store, ())?;

    Ok(())
}
