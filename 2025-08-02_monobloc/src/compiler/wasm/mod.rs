use crate::compiler::wasm::{
    code::Code,
    export::{Export, ExportDesc, Name},
    func_idx::FuncIdx,
    magic::Magic,
    section::emit_section,
    type_idx::TypeIdx,
    type_section::TypeSection,
    vec::emit_vec,
    version::Version,
};

mod code;
mod export;
mod func_idx;
mod func_type;
mod leb128;
mod magic;
mod section;
mod type_idx;
mod type_section;
mod vec;
mod version;

pub fn compile_module(_: i32) -> Vec<u8> {
    let mut output = Vec::new();

    Magic.emit(&mut output);
    Version.emit(&mut output);
    TypeSection.emit(&mut output);
    emit_function_section(&mut output);
    emit_export_section(&mut output);
    emit_code_section(&mut output);

    output
}

pub fn emit_function_section(output: &mut Vec<u8>) {
    let id = 3;

    let mut contents = Vec::new();
    emit_vec(&[TypeIdx { index: 0 }], &mut contents);

    emit_section(id, contents, output);
}

fn emit_export_section(output: &mut Vec<u8>) {
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

fn emit_code_section(output: &mut Vec<u8>) {
    let id = 10;

    let mut contents = Vec::new();
    emit_vec(&[Code {}], &mut contents);

    emit_section(id, contents, output);
}

trait Emit {
    fn emit(&self, output: &mut Vec<u8>);
}

impl Emit for u8 {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(*self);
    }
}
