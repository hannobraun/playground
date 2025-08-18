use std::{fs::File, io::Write};

use anyhow::Context;

use crate::{
    compiler::{ir::compile_input_code, tokens::tokenize, wasm},
    tests::read_input_code,
};

mod compiler;
mod runtime;
mod tests;

fn main() -> anyhow::Result<()> {
    let path = "examples/single-number.mbl";

    let input_code = read_input_code(path)?;
    println!("Input code:\n{input_code}");

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

    print!("Output: ");
    for (i, value) in stack.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }

        print!("{value}");
    }
    println!();

    Ok(())
}
