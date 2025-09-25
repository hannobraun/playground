/// # A StackAssembly program
pub struct Program {
    stack: Vec<i32>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(_code: &str) -> Self {
        Self { stack: Vec::new() }
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
    pub fn run(&mut self) {}
}
