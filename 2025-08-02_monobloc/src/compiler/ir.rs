pub struct Function {
    pub body: Vec<Expression>,
}

pub enum Expression {
    Value { value: i32 },
}
