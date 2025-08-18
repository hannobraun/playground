use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;

use crate::{
    compiler::{ir::compile_input_code, tokens::Tokenizer, wasm},
    runtime,
};

pub fn compile(path: &str) -> anyhow::Result<Vec<i32>> {
    let mut input_code = String::new();
    read_input_code(path, &mut input_code)?;
    let tokens = Tokenizer::new(&input_code).process_all_tokens();
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

pub fn read_input_code(path: &str, buf: &mut String) -> anyhow::Result<()> {
    File::open(path)
        .with_context(|| format!("Opening `{path}`"))?
        .read_to_string(buf)
        .with_context(|| format!("Reading code from `{path}`"))?;

    Ok(())
}
