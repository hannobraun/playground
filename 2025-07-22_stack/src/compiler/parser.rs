pub fn parse() -> anyhow::Result<Program> {
    Ok(Program {
        functions: vec![Function {
            name: "start",
            body: Expression::Literal { value: 42 },
        }],
    })
}

pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn number_of_functions(&self) -> u32 {
        let Ok(len) = self.functions.len().try_into() else {
            panic!("Number of functions can not be represented by `u32`.");
        };

        len
    }
}

pub struct Function {
    pub name: &'static str,
    pub body: Expression,
}

pub enum Expression {
    Literal { value: i32 },
}
