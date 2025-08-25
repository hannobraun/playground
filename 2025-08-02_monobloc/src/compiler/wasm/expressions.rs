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

        ir::Expression::Add => Instruction::I32Add,

        ir::Expression::Equals => Instruction::I32Eq,
        ir::Expression::GreaterThan => Instruction::I32GtS,
        ir::Expression::GreaterThanOrEquals => Instruction::I32GeS,
        ir::Expression::LessThan => Instruction::I32LtS,
        ir::Expression::LessThanOrEquals => Instruction::I32LeS,
        ir::Expression::Not => Instruction::I32Eqz,
    };

    instruction.emit(target);
}
