use crate::compiler::{
    ir,
    wasm::{Emit, instruction::Instruction},
};

pub struct Expressions;

impl Emit for Expressions {
    fn emit(&self, output: &mut Vec<u8>) {
        compile_expression(&ir::Expression::Value { value: 0 }, output);
        End.emit(output);
    }
}

fn compile_expression(expression: &ir::Expression, output: &mut Vec<u8>) {
    match *expression {
        ir::Expression::Value { value } => {
            Instruction::ConstI32 { value }.emit(output);
        }
    }
}

struct End;

impl Emit for End {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(0x0b);
    }
}
