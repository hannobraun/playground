use std::collections::BTreeMap;

use crate::{
    Effect,
    runtime::{CallStack, Instruction, Operands, Operator, StepOutcome, step},
};

/// # A StackAssembly program
pub struct Program {
    instructions: Vec<Instruction>,
    labels: BTreeMap<String, i32>,
    operands: Operands,
    call_stack: CallStack,
    effect: Option<Effect>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(input: &str) -> Self {
        let mut instructions = Vec::new();
        let mut labels = BTreeMap::new();

        for word in input.split_whitespace() {
            if word == "call" {
                instructions.push(Instruction::Operator {
                    operator: Operator::Call,
                });
            } else if word == "call_if" {
                instructions.push(Instruction::Operator {
                    operator: Operator::CallIf,
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

                let address = {
                    let address = instructions.len();

                    let Ok(address) = address.try_into() else {
                        // This is okay for now, but it would be nicer to reject
                        // this when pushing to `instructions`.
                        panic!(
                            "Label `{label}` points to address `{address}`, \
                            which can't be represented as a signed 32-bit \
                            integer. Too much code!"
                        );
                    };

                    address
                };

                // This overwrites any previous label of the same name. Fine for
                // now, but it would be better if this were an error.
                labels.insert(label.to_string(), address);
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
            call_stack: CallStack::new(),
            effect: None,
        }
    }

    /// # Call [`Program::compile`], then [`Program::continue_`]
    pub fn compile_and_run(input: &str) -> Self {
        let mut program = Self::compile(input);
        program.continue_();

        program
    }

    /// # Access the operand stack
    pub fn operands(&self) -> &Vec<i32> {
        self.operands.inner()
    }

    /// # Access the call stack
    pub fn call_stack(&self) -> &Vec<usize> {
        self.call_stack.inner()
    }

    /// # Access the currently triggered effect
    pub fn effect(&self) -> Option<&Effect> {
        self.effect.as_ref()
    }

    /// # Run the program until completion
    pub fn continue_(&mut self) {
        loop {
            match step(
                &self.instructions,
                &self.labels,
                &mut self.operands,
                &mut self.call_stack,
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
