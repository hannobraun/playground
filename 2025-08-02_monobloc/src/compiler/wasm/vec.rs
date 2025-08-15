use crate::compiler::wasm::{Emit, leb128};

pub fn emit_vec(items: &[impl Emit], output: &mut Vec<u8>) {
    emit_vec_length(items.len(), output);

    for item in items {
        item.emit(output);
    }
}

fn emit_vec_length(length: usize, output: &mut Vec<u8>) {
    let Ok(length) = length.try_into() else {
        panic!("Tried to emit vector length that is too large.");
    };

    leb128::emit_u32(length, output);
}
