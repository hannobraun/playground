use crate::compiler::{
    ir,
    wasm::{
        Emit, code_section::CodeSection, export_section::ExportSection,
        function_section::FunctionSection, magic::Magic,
        type_section::TypeSection, version::Version,
    },
};

pub struct Module<'a> {
    pub package: &'a ir::Package,
}

impl Emit for Module<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        Magic.emit(target);
        Version.emit(target);
        TypeSection {
            signature: &self.package.root().signature,
        }
        .emit(target);
        FunctionSection.emit(target);
        ExportSection.emit(target);
        CodeSection {
            root: self.package.root(),
        }
        .emit(target);
    }
}
