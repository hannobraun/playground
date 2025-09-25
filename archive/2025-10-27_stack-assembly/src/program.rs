/// # A StackAssembly program
pub struct Program {
    stack: Vec<i32>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(_code: &str) -> Self {
        Self { stack: Vec::new() }
    }

    /// # Access the program's stack
    pub fn stack(&self) -> &Vec<i32> {
        &self.stack
    }

    /// # Run the program until completion
    pub fn run(&mut self) {}
}
