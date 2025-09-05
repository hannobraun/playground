use crate::compiler::wasm::{
    Emit, section::Section, type_idx::TypeIdx, vec::WasmVec,
};

pub struct FunctionSection;

impl Emit for FunctionSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        TypeIdxVec {}.emit(&mut contents);

        Section {
            id: 3,
            contents: &contents,
        }
        .emit(target);
    }
}

struct TypeIdxVec {}

impl Emit for TypeIdxVec {
    fn emit(&self, target: &mut Vec<u8>) {
        WasmVec {
            items: &[TypeIdx { index: 0 }],
        }
        .emit(target);
    }
}
