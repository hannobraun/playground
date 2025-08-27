use crate::compiler::{
    intrinsics::Intrinsic,
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
    let ir::Expression::Intrinsic {
        intrinsic: expression,
    } = expression;

    let instruction = match *expression {
        Intrinsic::Assert => Instruction::If {
            block_type: BlockType::Empty,
            then: vec![],
            else_: vec![Instruction::Unreachable],
        },
        Intrinsic::Panic => Instruction::Unreachable,

        Intrinsic::Integer { value } => Instruction::I32Const { value },

        Intrinsic::Equals => Instruction::I32Eq,
        Intrinsic::GreaterThan => Instruction::I32GtS,
        Intrinsic::GreaterThanOrEquals => Instruction::I32GeS,
        Intrinsic::LessThan => Instruction::I32LtS,
        Intrinsic::LessThanOrEquals => Instruction::I32LeS,
        Intrinsic::Not => Instruction::I32Eqz,

        Intrinsic::Add => Instruction::I32Add,
        Intrinsic::Divide => Instruction::I32DivS,
        Intrinsic::Multiply => Instruction::I32Mul,
        Intrinsic::Remainder => Instruction::I32RemS,
        Intrinsic::Subtract => Instruction::I32Sub,

        Intrinsic::And => Instruction::I32And,
        Intrinsic::CountOnes => Instruction::I32Popcnt,
        Intrinsic::LeadingZeros => Instruction::I32Clz,
        Intrinsic::Or => Instruction::I32Or,
        Intrinsic::RotateLeft => Instruction::I32Rotl,
        Intrinsic::RotateRight => Instruction::I32Rotr,
        Intrinsic::ShiftLeft => Instruction::I32Shl,
        Intrinsic::ShiftRight => Instruction::I32ShrS,
        Intrinsic::TrailingZeros => Instruction::I32Ctz,
        Intrinsic::Xor => Instruction::I32Xor,
    };

    instruction.emit(target);
}
