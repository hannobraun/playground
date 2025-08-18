pub fn emit_version(output: &mut Vec<u8>) {
    output.extend([1, 0, 0, 0]);
}
