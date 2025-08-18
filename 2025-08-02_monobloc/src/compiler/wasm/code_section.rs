use crate::compiler::wasm::{
    Emit, code::Code, section::emit_section, vec::emit_vec,
};

pub struct CodeSection;

impl Emit for CodeSection {
    fn emit(&self, output: &mut Vec<u8>) {
        let id = 10;

        let mut contents = Vec::new();
        emit_vec(&[Code {}], &mut contents);

        emit_section(id, contents, output);
    }
}
