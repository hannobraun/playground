use crate::compiler::wasm::{code::Code, section::emit_section, vec::emit_vec};

pub fn emit_code_section(output: &mut Vec<u8>) {
    let id = 10;

    let mut contents = Vec::new();
    emit_vec(&[Code {}], &mut contents);

    emit_section(id, contents, output);
}
