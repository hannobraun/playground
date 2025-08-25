use crate::compiler::{
    ir,
    wasm::{
        Emit,
        instruction::{BlockType, End, Instruction},
    },
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
        ir::Expression::Assert => Instruction::If {
            block_type: BlockType::Empty,
            then: vec![],
            else_: vec![Instruction::Unreachable],
        },
        ir::Expression::Value { value } => Instruction::I32Const { value },
        ir::Expression::Equals => Instruction::EqI32,
    };

    instruction.emit(target);
}
