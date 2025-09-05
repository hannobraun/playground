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
        let package = self.package;

        Magic.emit(target);
        Version.emit(target);
        TypeSection { package }.emit(target);
        FunctionSection.emit(target);
        ExportSection.emit(target);
        CodeSection {
            root: package.root(),
        }
        .emit(target);
    }
}
