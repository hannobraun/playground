pub fn emit_magic(output: &mut Vec<u8>) {
    output.extend(b"\0asm");
}
