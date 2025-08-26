use crate::compiler::wasm::{Emit, leb128::Leb128};

pub enum Instruction {
    Unreachable,
    If {
        block_type: BlockType,
        then: Vec<Instruction>,
        else_: Vec<Instruction>,
    },

    I32Const {
        value: i32,
    },

    I32Eqz,
    I32Eq,
    I32LtS,
    I32GtS,
    I32LeS,
    I32GeS,

    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32RemS,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32Rotl,
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

            Self::I32Const { value } => {
                target.push(0x41);
                Leb128::I32 { value }.emit(target);
            }

            Self::I32Eqz => {
                target.push(0x45);
            }
            Self::I32Eq => {
                target.push(0x46);
            }
            Self::I32LtS => {
                target.push(0x48);
            }
            Self::I32GtS => {
                target.push(0x4a);
            }
            Self::I32LeS => {
                target.push(0x4c);
            }
            Self::I32GeS => {
                target.push(0x4e);
            }

            Self::I32Clz => {
                target.push(0x67);
            }
            Self::I32Ctz => {
                target.push(0x68);
            }
            Self::I32Popcnt => {
                target.push(0x69);
            }
            Self::I32Add => {
                target.push(0x6a);
            }
            Self::I32Sub => {
                target.push(0x6b);
            }
            Self::I32Mul => {
                target.push(0x6c);
            }
            Self::I32DivS => {
                target.push(0x6d);
            }
            Self::I32RemS => {
                target.push(0x6f);
            }
            Self::I32And => {
                target.push(0x71);
            }
            Self::I32Or => {
                target.push(0x72);
            }
            Self::I32Xor => {
                target.push(0x73);
            }
            Self::I32Shl => {
                target.push(0x74);
            }
            Self::I32ShrS => {
                target.push(0x75);
            }
            Self::I32Rotl => {
                target.push(0x77);
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
