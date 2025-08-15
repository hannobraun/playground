pub fn emit_usize(value: usize, output: &mut Vec<u8>) -> anyhow::Result<()> {
    let value = value
        .try_into()
        .expect("Can always convert `usize` to `u64`.");

    leb128::write::unsigned(output, value)?;

    Ok(())
}
