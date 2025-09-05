use crate::compiler::wasm::{Emit, leb128::Leb128};

pub struct Section<'a> {
    pub id: u8,
    pub contents: &'a [u8],
}

impl Emit for Section<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        SectionId { id: self.id }.emit(target);
        emit_section_size(self.contents.len(), target);
        emit_section_contents(self.contents, target);
    }
}

struct SectionId {
    id: u8,
}

impl Emit for SectionId {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(self.id);
    }
}

fn emit_section_size(size: usize, target: &mut Vec<u8>) {
    let Ok(size) = size.try_into() else {
        panic!("Unsupported section size: `{size}`");
    };

    Leb128::U32 { value: size }.emit(target);
}

fn emit_section_contents(contents: &[u8], target: &mut Vec<u8>) {
    target.extend(contents);
}
