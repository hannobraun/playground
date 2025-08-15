use crate::compiler::wasm::{section::emit_section, vec::emit_empty_vec};

mod section;
mod vec;

pub fn compile_wasm_module() -> anyhow::Result<Vec<u8>> {
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
    let id = 1;

    let mut contents = Vec::new();
    emit_empty_vec(&mut contents)?;

    emit_section(id, contents, output)?;

    Ok(())
}
