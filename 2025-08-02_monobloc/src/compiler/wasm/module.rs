use crate::compiler::{
    ir,
    wasm::{
        Emit, code_section::CodeSection, export_section::ExportSection,
        function_section::FunctionSection, type_section::TypeSection,
    },
};

pub struct Module<'r> {
    pub package: &'r ir::Package,
}

impl Emit for Module<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let blocks = &self.package.blocks;

        Magic.emit(target);
        Version.emit(target);
        TypeSection { blocks }.emit(target);
        FunctionSection { blocks }.emit(target);
        ExportSection.emit(target);
        CodeSection { blocks }.emit(target);
    }
}

pub struct Magic;

impl Emit for Magic {
    fn emit(&self, target: &mut Vec<u8>) {
        target.extend(b"\0asm");
    }
}

pub struct Version;

impl Emit for Version {
    fn emit(&self, target: &mut Vec<u8>) {
        target.extend([1, 0, 0, 0]);
    }
}
