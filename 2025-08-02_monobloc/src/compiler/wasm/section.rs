use crate::compiler::wasm::leb128;

pub fn emit_section(id: u8, contents: Vec<u8>, output: &mut Vec<u8>) {
    emit_section_id(id, output);
    emit_section_size(contents.len(), output);
    emit_section_contents(contents, output);
}

fn emit_section_id(id: u8, output: &mut Vec<u8>) {
    output.push(id);
}

fn emit_section_size(size: usize, output: &mut Vec<u8>) {
    leb128::emit_usize(size, output);
}

fn emit_section_contents(contents: Vec<u8>, output: &mut Vec<u8>) {
    output.extend(contents);
}
