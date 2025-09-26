/// # A StackAssembly program
pub struct Program {
    instructions: Vec<Instruction>,
    stack: Vec<i32>,
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
        let mut current_instruction = 0;

        loop {
            let Some(instruction) = self.instructions.get(current_instruction)
            else {
                break;
            };

            match instruction {
                Instruction::Operator {
                    operator: Operator::Integer { value },
                } => {
                    self.stack.push(*value);
                }
                Instruction::Operator {
                    operator: Operator::Unknown,
                } => {
                    self.effect = Some(Effect::UnknownOperator);
                    break;
                }
            }

            current_instruction += 1;
        }
    }
}

/// An effect that may be triggered by a program
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to evaluate an unknown operator
    UnknownOperator,
}

enum Instruction {
    Operator { operator: Operator },
}

enum Operator {
    Integer { value: i32 },
    Unknown,
}
