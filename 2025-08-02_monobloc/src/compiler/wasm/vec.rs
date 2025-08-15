use crate::compiler::wasm::{Emit, leb128::emit_u32};

pub fn emit_vec(items: &[impl Emit], output: &mut Vec<u8>) {
    emit_vec_length(items.len(), output);

    for item in items {
        item.emit(output);
    }
}

fn emit_vec_length(length: usize, output: &mut Vec<u8>) {
    let Ok(length) = length.try_into() else {
        panic!("Unsupported vector length: `{length}`");
    };

    emit_u32(length, output);
}
