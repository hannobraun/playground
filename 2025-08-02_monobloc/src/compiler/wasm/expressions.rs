use crate::compiler::wasm::{Emit, instruction::Instruction};

pub struct Expressions;

impl Emit for Expressions {
    fn emit(&self, output: &mut Vec<u8>) {
        compile_expression(output);
        End.emit(output);
    }
}

fn compile_expression(output: &mut Vec<u8>) {
    Instruction::ConstI32 { value: 0 }.emit(output);
}

struct End;

impl Emit for End {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(0x0b);
    }
}
