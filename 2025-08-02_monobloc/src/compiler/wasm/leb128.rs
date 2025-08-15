pub fn emit_usize(value: usize, output: &mut Vec<u8>) {
    let value = value
        .try_into()
        .expect("Can always convert `usize` to `u64`.");

    leb128::write::unsigned(output, value)
        .expect("Writing into a `&mut Vec` should never fail.");
}
