use std::collections::VecDeque;

use crate::compiler::tokenizer::Token;

pub fn parse(mut tokens: VecDeque<Token>) -> anyhow::Result<Program> {
    let mut program = Program {
        functions: Vec::new(),
    };

    while let Some(token) = tokens.pop_front() {
        let Token::Fun = token else {
            anyhow::bail!("Expected `fun` token.");
        };

        let Some(Token::Identifier { value: name }) = tokens.pop_front() else {
            anyhow::bail!("Expected identifier.");
        };

        let Some(Token::Literal { value: body }) = tokens.pop_front() else {
            anyhow::bail!("Expected expression.");
        };

        program.functions.push(Function {
            name: name.to_string(),
            body: Expression::Literal { value: body },
        })
    }

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
