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
        ir::Expression::Assert => Instruction::If {
            block_type: BlockType::Empty,
            then: vec![],
            else_: vec![Instruction::Unreachable],
        },
        ir::Expression::Panic => Instruction::Unreachable,

        ir::Expression::Integer { value } => Instruction::I32Const { value },

        ir::Expression::Equals => Instruction::I32Eq,
        ir::Expression::GreaterThan => Instruction::I32GtS,
        ir::Expression::GreaterThanOrEquals => Instruction::I32GeS,
        ir::Expression::LessThan => Instruction::I32LtS,
        ir::Expression::LessThanOrEquals => Instruction::I32LeS,
        ir::Expression::Not => Instruction::I32Eqz,

        ir::Expression::Add => Instruction::I32Add,
        ir::Expression::Divide => Instruction::I32DivS,
        ir::Expression::Multiply => Instruction::I32Mul,
        ir::Expression::Remainder => Instruction::I32RemS,
        ir::Expression::Subtract => Instruction::I32Sub,

        ir::Expression::And => Instruction::I32And,
        ir::Expression::CountOnes => Instruction::I32Popcnt,
        ir::Expression::LeadingZeros => Instruction::I32Clz,
        ir::Expression::Or => Instruction::I32Or,
        ir::Expression::RotateLeft => Instruction::I32Rotl,
        ir::Expression::RotateRight => Instruction::I32Rotr,
        ir::Expression::ShiftLeft => Instruction::I32Shl,
        ir::Expression::ShiftRight => Instruction::I32ShrS,
        ir::Expression::TrailingZeros => Instruction::I32Ctz,
        ir::Expression::Xor => Instruction::I32Xor,
    };

    instruction.emit(target);
}
