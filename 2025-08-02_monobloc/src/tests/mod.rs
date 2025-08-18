use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;

use crate::{
    compiler::{
        input_code::InputCode, ir::compile_input_code, tokens::Tokenizer, wasm,
    },
    runtime,
};

pub fn compile(path: &str) -> anyhow::Result<Vec<i32>> {
    // We wouldn't need to create the buffer here, if `String::into_chars` were
    // stable:
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_chars
    let mut input_code = String::new();
    let input_code = read_input_code(path, &mut input_code)?;
    let tokens = Tokenizer::new().process_all_tokens(input_code);
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

pub fn read_input_code<'a>(
    path: &str,
    buf: &'a mut String,
) -> anyhow::Result<InputCode<'a>> {
    File::open(path)
        .with_context(|| format!("Opening `{path}`"))?
        .read_to_string(buf)
        .with_context(|| format!("Reading code from `{path}`"))?;

    Ok(buf.chars().peekable())
}
