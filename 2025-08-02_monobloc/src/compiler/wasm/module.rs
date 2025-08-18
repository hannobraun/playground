use crate::compiler::{
    ir,
    wasm::{
        Emit, code_section::CodeSection, export_section::ExportSection,
        function_section::FunctionSection, magic::Magic,
        type_section::TypeSection, version::Version,
    },
};

pub struct Module<'a> {
    pub function: &'a ir::Function,
}

impl Emit for Module<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        Magic.emit(target);
        Version.emit(target);
        TypeSection {
            signature: &ir::Signature {
                inputs: vec![],
                outputs: vec![ir::Type::I32],
            },
        }
        .emit(target);
        FunctionSection.emit(target);
        ExportSection.emit(target);
        CodeSection {
            function: &self.function.body,
        }
        .emit(target);
    }
}
