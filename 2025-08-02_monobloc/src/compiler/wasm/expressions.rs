use crate::compiler::{
    ir,
    wasm::{Emit, instruction::Instruction},
};

pub struct Expressions<'a> {
    pub inner: &'a [ir::Expression],
}

impl Emit for Expressions<'_> {
    fn emit(&self, output: &mut Vec<u8>) {
        for expression in self.inner {
            compile_expression(expression, output);
        }

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
