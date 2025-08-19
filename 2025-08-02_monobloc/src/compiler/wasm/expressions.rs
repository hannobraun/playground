use crate::compiler::{
    ir,
    wasm::{Emit, instruction::Instruction},
};

pub struct Expressions<'a> {
    pub body: &'a ir::Body,
}

impl Emit for Expressions<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        for expression in self.body {
            compile_expression(expression, target);
        }

        End.emit(target);
    }
}

fn compile_expression(expression: &ir::Expression, target: &mut Vec<u8>) {
    let instruction = match *expression {
        ir::Expression::Panic => Instruction::Unreachable,
        ir::Expression::Value { value } => Instruction::ConstI32 { value },
        ir::Expression::Equals => Instruction::EqI32,
    };

    instruction.emit(target);
}

struct End;

impl Emit for End {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x0b);
    }
}
