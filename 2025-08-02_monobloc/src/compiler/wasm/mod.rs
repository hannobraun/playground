use crate::compiler::wasm::{
    functions::FuncType, section::emit_section, vec::emit_vec,
};

mod functions;
mod leb128;
mod section;
mod vec;

pub fn compile_module() -> Vec<u8> {
    let mut output = Vec::new();

    emit_magic(&mut output);
    emit_version(&mut output);
    emit_type_section(&mut output);
    emit_function_section(&mut output);

    output
}

fn emit_magic(output: &mut Vec<u8>) {
    output.extend(b"\0asm");
}

fn emit_version(output: &mut Vec<u8>) {
    output.extend([1, 0, 0, 0]);
}

fn emit_type_section(output: &mut Vec<u8>) {
    let id = 1;

    let mut contents = Vec::new();
    emit_vec(&[FuncType {}], &mut contents);

    emit_section(id, contents, output);
}

fn emit_function_section(output: &mut Vec<u8>) {
    let id = 3;

    let mut contents = Vec::new();
    let functions = &[];
    emit_vec(functions, &mut contents);

    emit_section(id, contents, output);
}

trait Emit {
    fn emit(&self, output: &mut Vec<u8>);
}
