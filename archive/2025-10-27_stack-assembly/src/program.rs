/// # A StackAssembly program
pub struct Program {
    code: String,
    stack: Vec<i32>,
    effect: Option<Effect>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(input: &str) -> Self {
        Self {
            code: input.to_string(),
            stack: Vec::new(),
            effect: None,
        }
    }

    /// # Call [`Program::compile`], then [`Program::run`]
    pub fn compile_and_run(input: &str) -> Self {
        let mut program = Self::compile(input);
        program.run();

        program
    }

    /// # Access the operand stack
    pub fn stack(&self) -> &Vec<i32> {
        &self.stack
    }

    /// # Access the currently triggered effect
    pub fn effect(&self) -> Option<&Effect> {
        self.effect.as_ref()
    }

    /// # Run the program until completion
    pub fn run(&mut self) {
        for word in self.code.split_whitespace() {
            let Ok(value) = word.parse() else {
                self.effect = Some(Effect::UnknownOperator);
                break;
            };

            self.stack.push(value);
        }
    }
}

/// An effect that may be triggered by a program
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to evaluate an unknown operator
    UnknownOperator,
}
