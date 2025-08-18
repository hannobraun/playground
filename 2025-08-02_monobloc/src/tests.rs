use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;

use crate::{
    compiler::{ir::compile_input_code, tokens::tokenize, wasm},
    runtime,
};

pub fn compile(path: &str) -> anyhow::Result<Vec<i32>> {
    let input_code = read_input_code(path)?;
    let tokens = tokenize(&input_code);
    let root = compile_input_code(tokens);
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

    Ok(stack)
}

pub fn read_input_code(path: &str) -> anyhow::Result<String> {
    let mut buf = String::new();

    File::open(path)
        .with_context(|| format!("Opening `{path}`"))?
        .read_to_string(&mut buf)
        .with_context(|| format!("Reading code from `{path}`"))?;

    Ok(buf)
}
