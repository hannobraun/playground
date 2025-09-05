use crate::compiler::wasm::{Emit, leb128::Leb128};

pub struct Section<'a> {
    pub id: u8,
    pub contents: &'a [u8],
}

impl Emit for Section<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        SectionId { id: self.id }.emit(target);
        SectionSize {
            size: self.contents.len(),
        }
        .emit(target);
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

fn emit_section_contents(contents: &[u8], target: &mut Vec<u8>) {
    target.extend(contents);
}
