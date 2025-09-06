use crate::compiler::wasm::{Emit, leb128::Leb128};

mod code_section;
mod export_section;
mod function_section;

pub use self::{
    code_section::CodeSection, export_section::ExportSection,
    function_section::FunctionSection,
};

pub struct Section<'r> {
    pub id: u8,
    pub contents: &'r [u8],
}

impl Emit for Section<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(self.id);

        SectionSize {
            size: self.contents.len(),
        }
        .emit(target);

        target.extend(self.contents);
    }
}

struct SectionSize {
    size: usize,
}

impl Emit for SectionSize {
    fn emit(&self, target: &mut Vec<u8>) {
        let Ok(size) = self.size.try_into() else {
            panic!("Unsupported section size: `{size}`", size = self.size);
        };

        Leb128::U32 { value: size }.emit(target);
    }
}
