use crate::compiler::{
    ir::types::Signature,
    wasm::{Emit, func_type::FuncType, section::emit_section, vec::WasmVec},
};

pub struct TypeSection<'a> {
    pub signature: &'a Signature,
}

impl Emit for TypeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 1;

        let mut contents = Vec::new();
        WasmVec {
            items: &[FuncType {
                signature: self.signature,
            }],
        }
        .emit(&mut contents);

        emit_section(id, contents, target);
    }
}
