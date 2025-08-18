pub fn compile_input_code() -> Function {
    Function {
        body: vec![Expression::Value { value: 1 }],
    }
}

pub struct Function {
    pub body: Body,
}

pub type Body = Vec<Expression>;

pub enum Expression {
    Value { value: i32 },
}
