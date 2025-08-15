use crate::compiler::wasm::{Emit, leb128::emit_u32, vec::emit_vec};

pub struct Code {}

impl Emit for Code {
    fn emit(&self, output: &mut Vec<u8>) {
        let mut func = Vec::new();
        emit_locals(&mut func);
        emit_expression(&mut func);

        let size = func.len();
        let Ok(size) = size.try_into() else {
            panic!("Unsupported code size: `{size}`");
        };

        emit_u32(size, output);
        output.extend(func);
    }
}

fn emit_locals(output: &mut Vec<u8>) {
    let locals: &[Code] = &[];
    emit_vec(locals, output);
}

fn emit_expression(output: &mut Vec<u8>) {
    emit_end(output);
}

fn emit_end(output: &mut Vec<u8>) {
    output.push(0x0b);
}
