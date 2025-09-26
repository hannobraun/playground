use crate::{
    Effect,
    runtime::{Evaluator, Instruction, Operator, StepOutcome},
};

/// # A StackAssembly program
pub struct Program {
    instructions: Vec<Instruction>,
    operands: Vec<i32>,
    effect: Option<Effect>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(input: &str) -> Self {
        let mut instructions = Vec::new();

        for word in input.split_whitespace() {
            if let Ok(value) = word.parse() {
                instructions.push(Instruction::Operator {
                    operator: Operator::Integer { value },
                });
            } else {
                instructions.push(Instruction::Operator {
                    operator: Operator::Unknown,
                });
            }
        }

        Self {
            instructions,
            operands: Vec::new(),
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
    pub fn operands(&self) -> &Vec<i32> {
        &self.operands
    }

    /// # Access the currently triggered effect
    pub fn effect(&self) -> Option<&Effect> {
        self.effect.as_ref()
    }

    /// # Run the program until completion
    pub fn run(&mut self) {
        let mut evaluator = Evaluator::new();

        loop {
            match evaluator.step(&self.instructions, &mut self.operands) {
                Ok(StepOutcome::Ready) => {
                    continue;
                }
                Ok(StepOutcome::Finished) => {
                    break;
                }
                Err(effect) => {
                    self.effect = Some(effect);
                    break;
                }
            }
        }
    }
}
