use crate::compiler::wasm::leb128;

pub fn emit_section(
    id: u8,
    contents: Vec<u8>,
    output: &mut Vec<u8>,
) -> anyhow::Result<()> {
    emit_section_id(id, output);
    emit_section_size(contents.len(), output)?;
    emit_section_contents(contents, output);

    Ok(())
}

fn emit_section_id(id: u8, output: &mut Vec<u8>) {
    output.push(id);
}

fn emit_section_size(size: usize, output: &mut Vec<u8>) -> anyhow::Result<()> {
    leb128::emit_usize(size, output)?;
    Ok(())
}

fn emit_section_contents(contents: Vec<u8>, output: &mut Vec<u8>) {
    output.extend(contents);
}
