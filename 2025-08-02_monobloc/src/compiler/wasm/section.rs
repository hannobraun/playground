use crate::compiler::wasm::{Emit, leb128::Leb128};

pub fn emit_section(id: u8, contents: Vec<u8>, target: &mut Vec<u8>) {
    emit_section_id(id, target);
    emit_section_size(contents.len(), target);
    emit_section_contents(contents, target);
}

fn emit_section_id(id: u8, target: &mut Vec<u8>) {
    target.push(id);
}

fn emit_section_size(size: usize, target: &mut Vec<u8>) {
    let Ok(size) = size.try_into() else {
        panic!("Unsupported section size: `{size}`");
    };

    Leb128::U32 { value: size }.emit(target);
}

fn emit_section_contents(contents: Vec<u8>, target: &mut Vec<u8>) {
    target.extend(contents);
}
