use std::{fs::File, io::Read};

use anyhow::Context;
use wasmtime::{Engine, Instance, Module, Store};

fn main() -> anyhow::Result<()> {
    let input_code = read_input_code("numbers.mbl")?;
    println!("{input_code}");

    let wasm_code = compile_wasm_module()?;
    run_wasm_module(&wasm_code)?;

    let mut stack = Vec::<i32>::new();

    for identifier in input_code.split_whitespace() {
        if let Ok(value) = identifier.parse() {
            stack.push(value);
        } else {
            println!("Unknown identifier: `{identifier}`");
        }
    }

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

fn compile_wasm_module() -> anyhow::Result<Vec<u8>> {
    let mut output = Vec::new();

    emit_magic(&mut output);
    emit_version(&mut output);
    emit_type_section(&mut output)?;

    Ok(output)
}

fn emit_magic(output: &mut Vec<u8>) {
    output.extend(b"\0asm");
}

fn emit_version(output: &mut Vec<u8>) {
    output.extend([1, 0, 0, 0]);
}

fn emit_type_section(output: &mut Vec<u8>) -> anyhow::Result<()> {
    let mut contents = Vec::new();
    emit_empty_vec(&mut contents)?;

    emit_section_id(1, output);
    emit_section_size(contents.len(), output)?;
    emit_section_contents(contents, output);

    Ok(())
}

fn emit_empty_vec(output: &mut Vec<u8>) -> anyhow::Result<()> {
    emit_vec_length(0, output)?;
    Ok(())
}

fn emit_vec_length(length: usize, output: &mut Vec<u8>) -> anyhow::Result<()> {
    let length = length
        .try_into()
        .expect("Can always conver `usize` to `u64`.");

    leb128::write::unsigned(output, length)?;

    Ok(())
}

fn emit_section_id(id: u8, output: &mut Vec<u8>) {
    output.push(id);
}

fn emit_section_size(size: usize, output: &mut Vec<u8>) -> anyhow::Result<()> {
    let size = size
        .try_into()
        .expect("Can always convert `usize` to `u64`.");

    leb128::write::unsigned(output, size)?;

    Ok(())
}

fn emit_section_contents(contents: Vec<u8>, output: &mut Vec<u8>) {
    output.extend(contents);
}

fn run_wasm_module(code: &[u8]) -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::new(&engine, code)?;
    let mut store = Store::new(&engine, ());
    Instance::new(&mut store, &module, &[])?;

    Ok(())
}
