pub fn emit_u32(value: u32, output: &mut Vec<u8>) {
    leb128::write::unsigned(output, value.into())
        .expect("Writing into a `&mut Vec` should never fail.");
}
