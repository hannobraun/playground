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
    pub fn number_of_functions(&self) -> anyhow::Result<u32> {
        let len = self.functions.len().try_into()?;
        Ok(len)
    }
}

pub struct Function {
    pub name: &'static str,
    pub body: Expression,
}

pub enum Expression {
    Literal { value: i32 },
}
