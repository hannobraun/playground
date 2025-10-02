use crate::{
    Effect,
    instructions::{Instruction, Instructions, Labels, Operator},
};

pub fn compile(input: &str) -> (Instructions, Labels) {
    let mut instructions = Instructions::new();
    let mut labels = Labels::new();

    for token in input.split_whitespace() {
        if let Some(reference) = parse_reference(token) {
            translate_reference(reference, &mut instructions);
        } else if let Some(label) = parse_label(token) {
            translate_label(label, &mut instructions, &mut labels);
        } else {
            let operator = parse_operator(token);
            translate_operator(operator, &mut instructions);
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

fn translate_label(
    name: &str,
    instructions: &mut Instructions,
    labels: &mut Labels,
) {
    // Encountering a label means that the previous function has ended.
    instructions.push(Instruction::Return);

    let address = {
        let address = instructions.len();

        let Ok(address) = address.try_into() else {
            // This is okay for now, but it would be nicer to reject
            // this when pushing to `instructions`.
            panic!(
                "Label `{name}` points to address `{address}`, which can't be \
                represented as a signed 32-bit integer. Too much code!"
            );
        };

        address
    };

    // This overwrites any previous label of the same name. Fine for now, but it
    // would be better if this were an error.
    labels.insert(name.to_string(), address);
}

fn translate_operator(
    operator: Option<Operator>,
    instructions: &mut Instructions,
) {
    if let Some(operator) = operator {
        instructions.push(Instruction::Operator { operator });
    } else {
        instructions.push(Instruction::Trigger {
            effect: Effect::UnknownOperator,
        })
    }
}

fn translate_reference(name: &str, instructions: &mut Instructions) {
    let name = name.to_string();
    instructions.push(Instruction::Reference { name });
}
