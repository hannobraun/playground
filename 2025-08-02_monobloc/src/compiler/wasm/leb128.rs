pub fn emit_usize(value: usize, output: &mut Vec<u8>) {
    let value = value
        .try_into()
        .expect("Can always convert `usize` to `u64`.");

    emit_u64(value, output);
}

pub fn emit_u32(value: u32, output: &mut Vec<u8>) {
    leb128::write::unsigned(output, value.into())
        .expect("Writing into a `&mut Vec` should never fail.");
}

pub fn emit_u64(value: u64, output: &mut Vec<u8>) {
    leb128::write::unsigned(output, value)
        .expect("Writing into a `&mut Vec` should never fail.");
}
