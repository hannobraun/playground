#[derive(Debug)]
pub enum Instruction {
    Operator { operator: Operator },
    Reference { name: String },
    Return,
}

#[derive(Debug)]
pub enum Operator {
    Integer { value: i32 },

    Call,
    CallIf,
    Drop0,
    Yield,

    Unknown,
}
