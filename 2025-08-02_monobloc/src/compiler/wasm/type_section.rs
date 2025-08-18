use crate::compiler::wasm::{
    func_type::FuncType, section::emit_section, vec::emit_vec,
};

pub fn emit_type_section(output: &mut Vec<u8>) {
    let id = 1;

    let mut contents = Vec::new();
    emit_vec(&[FuncType {}], &mut contents);

    emit_section(id, contents, output);
}
