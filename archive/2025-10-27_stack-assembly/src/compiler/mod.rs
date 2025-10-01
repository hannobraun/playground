use crate::instructions::{Instruction, Instructions, Labels, Operator};

pub fn compile(input: &str) -> (Instructions, Labels) {
    let mut instructions = Vec::new();
    let mut labels = Labels::new();

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
        } else if word == "yield" {
            instructions.push(Instruction::Operator {
                operator: Operator::Yield,
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

    (instructions, labels)
}
