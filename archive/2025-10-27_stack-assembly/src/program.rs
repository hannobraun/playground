/// # A StackAssembly program
pub struct Program {
    code: String,
    stack: Vec<i32>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(code: &str) -> Self {
        Self {
            code: code.to_string(),
            stack: Vec::new(),
        }
    }

    /// # Call [`Program::compile`], then [`Program::run`]
    pub fn compile_and_run(code: &str) -> Self {
        let mut program = Self::compile(code);
        program.run();

        program
    }

    /// # Access the program's stack
    pub fn stack(&self) -> &Vec<i32> {
        &self.stack
    }

    /// # Run the program until completion
    pub fn run(&mut self) {
        for word in self.code.split_whitespace() {
            let Ok(value) = word.parse() else {
                break;
            };

            self.stack.push(value);
        }
    }
}

/// An effect that may be triggered by a program
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {}
