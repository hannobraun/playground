pub fn emit_vec(items: &[()], output: &mut Vec<u8>) -> anyhow::Result<()> {
    assert_eq!(
        items.len(),
        0,
        "Only empty vectors are supported right now.",
    );
    emit_vec_length(items.len(), output)?;
    Ok(())
}

fn emit_vec_length(length: usize, output: &mut Vec<u8>) -> anyhow::Result<()> {
    let length = length
        .try_into()
        .expect("Can always convert `usize` to `u64`.");

    leb128::write::unsigned(output, length)?;

    Ok(())
}
