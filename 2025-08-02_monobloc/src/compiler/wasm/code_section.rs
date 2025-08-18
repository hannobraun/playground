use crate::compiler::wasm::{
    Emit, leb128::emit_u32, section::emit_section, vec::emit_vec,
};

pub struct CodeSection;

impl Emit for CodeSection {
    fn emit(&self, output: &mut Vec<u8>) {
        let id = 10;

        let mut contents = Vec::new();
        emit_vec(&[Code {}], &mut contents);

        emit_section(id, contents, output);
    }
}

pub struct Code {}

impl Emit for Code {
    fn emit(&self, output: &mut Vec<u8>) {
        let mut func = Vec::new();
        Locals.emit(&mut func);
        Expression.emit(&mut func);

        let size = func.len();
        let Ok(size) = size.try_into() else {
            panic!("Unsupported code size: `{size}`");
        };

        emit_u32(size, output);
        output.extend(func);
    }
}

struct Locals;

impl Emit for Locals {
    fn emit(&self, output: &mut Vec<u8>) {
        let locals: &[Code] = &[];
        emit_vec(locals, output);
    }
}

struct Expression;

impl Emit for Expression {
    fn emit(&self, output: &mut Vec<u8>) {
        emit_end(output);
    }
}

fn emit_end(output: &mut Vec<u8>) {
    output.push(0x0b);
}
