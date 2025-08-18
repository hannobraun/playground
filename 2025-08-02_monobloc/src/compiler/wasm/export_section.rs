use crate::compiler::wasm::{
    export::{Export, ExportDesc, Name},
    func_idx::FuncIdx,
    section::emit_section,
    vec::emit_vec,
};

pub fn emit_export_section(output: &mut Vec<u8>) {
    let id = 7;

    let mut contents = Vec::new();
    emit_vec(
        &[Export {
            name: Name { inner: "root" },
            desc: ExportDesc::FuncIdx {
                index: FuncIdx { index: 0 },
            },
        }],
        &mut contents,
    );

    emit_section(id, contents, output);
}
