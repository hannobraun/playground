use crate::compiler::wasm::{Emit, leb128::Leb128};

pub enum Instruction {
    Unreachable,
    If {
        block_type: BlockType,
        then: Vec<Instruction>,
        else_: Vec<Instruction>,
    },
    ConstI32 {
        value: i32,
    },
    EqI32,
}

impl Emit for Instruction {
    fn emit(&self, target: &mut Vec<u8>) {
        match *self {
            Self::Unreachable => {
                target.push(0x00);
            }
            Self::If {
                ref block_type,
                ref then,
                ref else_,
            } => {
                If.emit(target);
                block_type.emit(target);
                for instruction in then {
                    instruction.emit(target);
                }
                Else.emit(target);
                for instruction in else_ {
                    instruction.emit(target);
                }
                End.emit(target);
            }
            Self::ConstI32 { value } => {
                target.push(0x41);
                Leb128::I32 { value }.emit(target);
            }
            Self::EqI32 => {
                target.push(0x46);
            }
        }
    }
}

pub enum BlockType {
    Empty,
}

impl Emit for BlockType {
    fn emit(&self, target: &mut Vec<u8>) {
        match self {
            Self::Empty => {
                target.push(0x40);
            }
        }
    }
}

pub struct If;

impl Emit for If {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x04);
    }
}

pub struct Else;

impl Emit for Else {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x05);
    }
}

pub struct End;

impl Emit for End {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x0b);
    }
}
