use crate::compiler::{
    ir,
    wasm::{
        Emit,
        sections::{
            CodeSection, ExportSection, FunctionSection, TableSection,
            TypeSection,
        },
    },
};

pub struct Module<'r> {
    pub package: &'r ir::Package,
}

impl Emit for Module<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let signatures = &self.package.signatures;
        let blocks = &self.package.blocks;

        Magic.emit(target);
        Version.emit(target);
        TypeSection { signatures, blocks }.emit(target);
        FunctionSection { blocks }.emit(target);
        TableSection { blocks }.emit(target);
        ExportSection.emit(target);
        CodeSection { blocks }.emit(target);
    }
}

struct Magic;

impl Emit for Magic {
    fn emit(&self, target: &mut Vec<u8>) {
        target.extend(b"\0asm");
    }
}

struct Version;

impl Emit for Version {
    fn emit(&self, target: &mut Vec<u8>) {
        target.extend([1, 0, 0, 0]);
    }
}
