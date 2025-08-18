use crate::compiler::wasm::{
    Emit,
    export::{Export, ExportDesc, Name},
    func_idx::FuncIdx,
    section::emit_section,
    vec::WasmVec,
};

pub struct ExportSection;

impl Emit for ExportSection {
    fn emit(&self, output: &mut Vec<u8>) {
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

        emit_section(id, contents, output);
    }
}
