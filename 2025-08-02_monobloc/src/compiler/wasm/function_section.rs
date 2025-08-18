use crate::compiler::wasm::{
    Emit, section::emit_section, type_idx::TypeIdx, vec::WasmVec,
};

pub struct FunctionSection;

impl Emit for FunctionSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 3;

        let mut contents = Vec::new();
        WasmVec {
            items: &[TypeIdx { index: 0 }],
        }
        .emit(&mut contents);

        emit_section(id, contents, target);
    }
}
