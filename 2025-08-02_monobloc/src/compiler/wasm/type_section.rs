use crate::compiler::{
    ir,
    wasm::{Emit, func_type::FuncType, section::Section, vec::WasmVec},
};

pub struct TypeSection<'r> {
    pub package: &'r ir::Package,
}

impl Emit for TypeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        FuncTypeVec {
            package: self.package,
        }
        .emit(&mut contents);

        Section {
            id: 1,
            contents: &contents,
        }
        .emit(target);
    }
}

struct FuncTypeVec<'r> {
    package: &'r ir::Package,
}

impl Emit for FuncTypeVec<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        WasmVec {
            items: &[FuncType {
                signature: &self.package.root().signature,
            }],
        }
        .emit(target);
    }
}
