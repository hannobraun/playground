use crate::{
    Effect,
    compiler::parse::{Expression, Function},
    instructions::{Instruction, Instructions, Labels, Operator},
};

pub fn translate_function(
    name: Option<&str>,
    function: Function,
    instructions: &mut Instructions,
    labels: &mut Labels,
) {
    if let Some(name) = name {
        translate_label(name, instructions, labels);
    }

    for expression in function.body {
        match expression {
            Expression::Operator { operator } => {
                translate_operator(operator, instructions);
            }
            Expression::Reference { name } => {
                translate_reference(name, instructions);
            }
        }
    }
}

pub fn translate_label(
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

pub fn translate_operator(
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

pub fn translate_reference(name: &str, instructions: &mut Instructions) {
    let name = name.to_string();
    instructions.push(Instruction::Reference { name });
}
