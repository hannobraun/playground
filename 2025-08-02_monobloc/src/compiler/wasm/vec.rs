pub fn emit_empty_vec(output: &mut Vec<u8>) -> anyhow::Result<()> {
    emit_vec_length(0, output)?;
    Ok(())
}

fn emit_vec_length(length: usize, output: &mut Vec<u8>) -> anyhow::Result<()> {
    let length = length
        .try_into()
        .expect("Can always conver `usize` to `u64`.");

    leb128::write::unsigned(output, length)?;

    Ok(())
}
