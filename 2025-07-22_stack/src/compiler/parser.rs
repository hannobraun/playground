pub fn parse() -> anyhow::Result<Program> {
    let mut program = Program {
        functions: Vec::new(),
    };

    program.functions.push(Function {
        name: "start".to_string(),
        body: Expression::Literal { value: 42 },
    });

    Ok(program)
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
    pub name: String,
    pub body: Expression,
}

pub enum Expression {
    Literal { value: i32 },
}
