use crate::compiler::wasm::{
    Emit, section::emit_section, type_idx::TypeIdx, vec::emit_vec,
};

pub struct FunctionSection;

impl Emit for FunctionSection {
    fn emit(&self, output: &mut Vec<u8>) {
        let id = 3;

        let mut contents = Vec::new();
        emit_vec(&[TypeIdx { index: 0 }], &mut contents);

        emit_section(id, contents, output);
    }
}
