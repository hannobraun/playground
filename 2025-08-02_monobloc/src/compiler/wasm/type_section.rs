use crate::compiler::wasm::{
    Emit, func_type::FuncType, section::emit_section, vec::WasmVec,
};

pub struct TypeSection;

impl Emit for TypeSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 1;

        let mut contents = Vec::new();
        WasmVec {
            items: &[FuncType {}],
        }
        .emit(&mut contents);

        emit_section(id, contents, target);
    }
}
