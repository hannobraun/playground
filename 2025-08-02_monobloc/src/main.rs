use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;

use crate::compiler::{ir::compile_input_code, wasm};

mod compiler;
mod runtime;

fn main() -> anyhow::Result<()> {
    let input_code = read_input_code("examples/single-number.mbl")?;
    println!("{input_code}");

    let root = compile_input_code(&input_code);
    let wasm_code = wasm::compile_module(&root);
    let stack = match runtime::evaluate_root(&wasm_code, &root) {
        Ok(stack) => stack,
        Err(err) => {
            let output = "error.wasm";
            File::create(output)?.write_all(&wasm_code)?;
            return Err(err).with_context(|| {
                format!("Error evaluating root; wrote module to `{output}`")
            });
        }
    };

    print!("Compiler: ");
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
