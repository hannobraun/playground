use crate::compiler::{
    ir,
    wasm::{Emit, func_type::FuncType, section::emit_section, vec::WasmVec},
};

pub struct TypeSection<'a> {
    pub signature: &'a ir::Signature,
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
