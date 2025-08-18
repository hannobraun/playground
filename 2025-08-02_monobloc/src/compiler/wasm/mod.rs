use crate::compiler::wasm::{
    code_section::emit_code_section, export_section::ExportSection,
    function_section::FunctionSection, magic::Magic, type_section::TypeSection,
    version::Version,
};

mod code;
mod code_section;
mod export;
mod export_section;
mod func_idx;
mod func_type;
mod function_section;
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
    FunctionSection.emit(&mut output);
    ExportSection.emit(&mut output);
    emit_code_section(&mut output);

    output
}

trait Emit {
    fn emit(&self, output: &mut Vec<u8>);
}

impl Emit for u8 {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(*self);
    }
}
