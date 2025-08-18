use crate::compiler::{
    ir,
    wasm::{
        Emit, code_section::CodeSection, export_section::ExportSection,
        function_section::FunctionSection, magic::Magic,
        type_section::TypeSection, version::Version,
    },
};

pub struct Module;

impl Emit for Module {
    fn emit(&self, output: &mut Vec<u8>) {
        Magic.emit(output);
        Version.emit(output);
        TypeSection.emit(output);
        FunctionSection.emit(output);
        ExportSection.emit(output);
        CodeSection {
            function: &[ir::Expression::Value { value: 0 }],
        }
        .emit(output);
    }
}
