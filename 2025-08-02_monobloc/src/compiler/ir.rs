pub struct Function {
    pub body: Body,
}

pub type Body = Vec<Expression>;

pub enum Expression {
    Value { value: i32 },
}
