use std::collections::BTreeMap;

use crate::{
    Effect,
    runtime::{Evaluator, Instruction, Operands, Operator, StepOutcome},
};

/// # A StackAssembly program
pub struct Program {
    instructions: Vec<Instruction>,
    labels: BTreeMap<String, ()>,
    operands: Operands,
    effect: Option<Effect>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(input: &str) -> Self {
        let mut instructions = Vec::new();
        let mut labels = BTreeMap::new();

        for word in input.split_whitespace() {
            if word == "apply" {
                instructions.push(Instruction::Operator {
                    operator: Operator::Apply,
                });
            } else if word == "drop0" {
                instructions.push(Instruction::Operator {
                    operator: Operator::Drop0,
                });
            } else if let Some(("", reference)) = word.split_once("@") {
                instructions.push(Instruction::Reference {
                    name: reference.to_string(),
                });
            } else if let Some((label, "")) = word.rsplit_once(":") {
                // Encountering a label means that the previous function has
                // ended.
                instructions.push(Instruction::Return);

                labels.insert(label.to_string(), ());
            } else if let Ok(value) = word.parse() {
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
            labels,
            operands: Operands::new(),
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
        self.operands.inner()
    }

    /// # Access the currently triggered effect
    pub fn effect(&self) -> Option<&Effect> {
        self.effect.as_ref()
    }

    /// # Run the program until completion
    pub fn run(&mut self) {
        let mut evaluator = Evaluator::new();

        loop {
            match evaluator.step(
                &self.instructions,
                &self.labels,
                &mut self.operands,
            ) {
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
