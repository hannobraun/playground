use crate::compiler::tokenize::Token;

pub fn parse_script<'r>(
    tokens: impl IntoIterator<Item = Token<'r>>,
) -> Script<'r> {
    let mut tokens = tokens.into_iter();

    let (root, mut current_label) = Function::parse(&mut tokens);
    let mut functions = Vec::new();

    while let Some(name) = current_label {
        let (function, next_label) = Function::parse(&mut tokens);
        functions.push((name, function));
        current_label = next_label;
    }

    Script { root, functions }
}

pub struct Script<'r> {
    pub root: Function<'r>,
    pub functions: Vec<(&'r str, Function<'r>)>,
}

pub struct Function<'r> {
    pub body: Vec<Expression<'r>>,
}

impl<'r> Function<'r> {
    fn parse(
        tokens: &mut impl Iterator<Item = Token<'r>>,
    ) -> (Self, Option<&'r str>) {
        let mut function = Self { body: Vec::new() };

        loop {
            let Some(token) = tokens.next() else {
                // No more tokens, which means this is the last function.
                return (function, None);
            };

            let expression = match token {
                Token::Label { name } => {
                    // A label starts the next function. We're done here.
                    return (function, Some(name));
                }

                Token::Operator { name } => Expression::parse_operator(name),
                Token::Reference { name } => Expression::Reference { name },
            };

            function.body.push(expression);
        }
    }
}

pub enum Expression<'r> {
    Operator { operator: Option<Operator> },
    Reference { name: &'r str },
}

impl Expression<'_> {
    fn parse_operator(token: &str) -> Self {
        let operator = if token == "call" {
            Some(Operator::Call)
        } else if token == "call_if" {
            Some(Operator::CallIf)
        } else if token == "drop0" {
            Some(Operator::Drop0)
        } else if token == "yield" {
            Some(Operator::Yield)
        } else if let Ok(value) = token.parse() {
            Some(Operator::Integer { value })
        } else {
            None
        };

        Self::Operator { operator }
    }
}

#[derive(Debug)]
pub enum Operator {
    Integer { value: i32 },

    Call,
    CallIf,
    Drop0,
    Yield,
}
