use crate::compiler::wasm::leb128;

pub fn emit_vec(items: &[()], output: &mut Vec<u8>) -> anyhow::Result<()> {
    assert_eq!(
        items.len(),
        0,
        "Only empty vectors are supported right now.",
    );
    emit_vec_length(items.len(), output);
    Ok(())
}

fn emit_vec_length(length: usize, output: &mut Vec<u8>) {
    leb128::emit_usize(length, output);
}
