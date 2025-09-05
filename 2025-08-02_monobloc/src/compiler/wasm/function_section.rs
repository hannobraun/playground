use crate::compiler::wasm::{
    Emit, section::Section, type_idx::TypeIdx, vec::WasmVec,
};

pub struct FunctionSection;

impl Emit for FunctionSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        WasmVec {
            items: &[TypeIdx { index: 0 }],
        }
        .emit(&mut contents);

        Section {
            id: 3,
            contents: &contents,
        }
        .emit(target);
    }
}
