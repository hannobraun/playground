use std::{fs::File, io::Write};

use anyhow::Context;

use crate::{
    compiler::{
        input_code::read_input_code, ir::compile_tokens, tokens::Tokenizer,
        wasm,
    },
    runtime,
};

pub fn compile(path: &str, _: bool) -> anyhow::Result<Vec<i32>> {
    // We wouldn't need to create the buffer here, if `String::into_chars` were
    // stable:
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_chars
    let mut input_code = String::new();
    let input_code = read_input_code(path, &mut input_code)?;

    let tokens = Tokenizer::new().process_all_tokens(input_code);
    let root = compile_tokens(tokens);
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
