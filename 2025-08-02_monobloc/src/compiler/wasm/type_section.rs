use crate::compiler::{
    ir,
    wasm::{Emit, func_type::FuncType, section::Section, vec::WasmVec},
};

pub struct TypeSection<'r> {
    pub package: &'r ir::Package,
}

impl Emit for TypeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 1;

        let mut contents = Vec::new();
        WasmVec {
            items: &[FuncType {
                signature: &self.package.root().signature,
            }],
        }
        .emit(&mut contents);

        Section {
            id,
            contents: &contents,
        }
        .emit(target);
    }
}
