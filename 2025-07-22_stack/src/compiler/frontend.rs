pub fn parse() -> Program {
    Program {
        function: Function {
            name: "start",
            body: Expression::Literal { value: 42 },
        },
    }
}

pub struct Program {
    pub function: Function,
}

impl Program {
    pub fn number_of_functions(&self) -> u32 {
        1
    }
}

pub struct Function {
    pub name: &'static str,
    pub body: Expression,
}

pub enum Expression {
    Literal { value: i32 },
}
