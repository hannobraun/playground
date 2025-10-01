use std::collections::BTreeMap;

use crate::Effect;

pub type Instructions = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    Operator { operator: Operator },
    Reference { name: String },
    Return,
    Trigger { effect: Effect },
}

#[derive(Debug)]
pub enum Operator {
    Integer { value: i32 },

    Call,
    CallIf,
    Drop0,
    Yield,
}

pub type Labels = BTreeMap<String, i32>;
