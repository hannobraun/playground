use crate::{
    Effect,
    instructions::{Instruction, Instructions, Labels, Operator},
};

pub fn compile(input: &str) -> (Instructions, Labels) {
    let mut instructions = Vec::new();
    let mut labels = Labels::new();

    for token in input.split_whitespace() {
        if let Some(reference) = parse_reference(token) {
            instructions.push(Instruction::Reference {
                name: reference.to_string(),
            });
        } else if let Some(label) = parse_label(token) {
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
        } else if let Some(operator) = parse_operator(token) {
            instructions.push(Instruction::Operator { operator });
        } else {
            instructions.push(Instruction::Trigger {
                effect: Effect::UnknownOperator,
            })
        }
    }

    (instructions, labels)
}

fn parse_label(token: &str) -> Option<&str> {
    if let Some((label, "")) = token.rsplit_once(":") {
        Some(label)
    } else {
        None
    }
}

fn parse_operator(token: &str) -> Option<Operator> {
    let operator = if token == "call" {
        Operator::Call
    } else if token == "call_if" {
        Operator::CallIf
    } else if token == "drop0" {
        Operator::Drop0
    } else if token == "yield" {
        Operator::Yield
    } else if let Ok(value) = token.parse() {
        Operator::Integer { value }
    } else {
        return None;
    };

    Some(operator)
}

fn parse_reference(token: &str) -> Option<&str> {
    if let Some(("", reference)) = token.split_once("@") {
        Some(reference)
    } else {
        None
    }
}
