use crate::compiler::{
    ir::{self},
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
    let ir::Expression::Intrinsic { intrinsic } = expression;

    let instruction = match *intrinsic {
        ir::Intrinsic::Assert => Instruction::If {
            block_type: BlockType::Empty,
            then: vec![],
            else_: vec![Instruction::Unreachable],
        },
        ir::Intrinsic::Panic => Instruction::Unreachable,

        ir::Intrinsic::Integer { value } => Instruction::I32Const { value },

        ir::Intrinsic::Equals => Instruction::I32Eq,
        ir::Intrinsic::GreaterThan => Instruction::I32GtS,
        ir::Intrinsic::GreaterThanOrEquals => Instruction::I32GeS,
        ir::Intrinsic::LessThan => Instruction::I32LtS,
        ir::Intrinsic::LessThanOrEquals => Instruction::I32LeS,
        ir::Intrinsic::Not => Instruction::I32Eqz,

        ir::Intrinsic::Add => Instruction::I32Add,
        ir::Intrinsic::Divide => Instruction::I32DivS,
        ir::Intrinsic::Multiply => Instruction::I32Mul,
        ir::Intrinsic::Remainder => Instruction::I32RemS,
        ir::Intrinsic::Subtract => Instruction::I32Sub,

        ir::Intrinsic::And => Instruction::I32And,
        ir::Intrinsic::CountOnes => Instruction::I32Popcnt,
        ir::Intrinsic::LeadingZeros => Instruction::I32Clz,
        ir::Intrinsic::Or => Instruction::I32Or,
        ir::Intrinsic::RotateLeft => Instruction::I32Rotl,
        ir::Intrinsic::RotateRight => Instruction::I32Rotr,
        ir::Intrinsic::ShiftLeft => Instruction::I32Shl,
        ir::Intrinsic::ShiftRight => Instruction::I32ShrS,
        ir::Intrinsic::TrailingZeros => Instruction::I32Ctz,
        ir::Intrinsic::Xor => Instruction::I32Xor,
    };

    instruction.emit(target);
}
