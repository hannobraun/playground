use crate::compiler::wasm::{
    Emit,
    export::{Export, ExportDesc, Name},
    func_idx::FuncIdx,
    section::Section,
    vec::WasmVec,
};

pub struct ExportSection;

impl Emit for ExportSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 7;

        let mut contents = Vec::new();
        WasmVec {
            items: &[Export {
                name: Name { inner: "root" },
                desc: ExportDesc::FuncIdx {
                    index: FuncIdx { index: 0 },
                },
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
