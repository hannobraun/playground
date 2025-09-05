use crate::compiler::{
    ir,
    wasm::{
        Emit, code_section::CodeSection, export_section::ExportSection,
        function_section::FunctionSection, magic::Magic,
        type_section::TypeSection, version::Version,
    },
};

pub struct Module<'r> {
    pub package: &'r ir::Package,
}

impl Emit for Module<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let package = self.package;

        Magic.emit(target);
        Version.emit(target);
        TypeSection {
            blocks: &package.blocks,
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
