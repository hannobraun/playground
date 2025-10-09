use crate::{
    Effect,
    compiler::parse::{Expression, Function, Operator, Script},
    instructions::{Instruction, Instructions, Labels},
    value::Value,
};

pub fn translate_script(script: Script) -> (Instructions, Labels) {
    let mut instructions = Instructions::new();
    let mut labels = Labels::new();

    let name = None;
    translate_function(name, script.root, &mut instructions, &mut labels);

    for (name, function) in script.functions {
        translate_function(
            Some(name),
            function,
            &mut instructions,
            &mut labels,
        );
    }

    (instructions, labels)
}

fn translate_function(
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

        Value { inner: address }
    };

    // This overwrites any previous label of the same name. Fine for now, but it
    // would be better if this were an error.
    labels.insert(name.to_string(), address);
}

fn translate_operator(
    operator: Option<Operator>,
    instructions: &mut Instructions,
) {
    let Some(operator) = operator else {
        instructions.push(Instruction::Trigger {
            effect: Effect::UnknownOperator,
        });
        return;
    };

    match operator {
        Operator::Integer { value } => {
            instructions.push(Instruction::PushValue { value });
        }
        Operator::Call => {
            instructions.push(Instruction::PushReturnAddress);
            instructions.push(Instruction::Jump);
        }
        Operator::CallIf => {
            instructions.push(Instruction::PushReturnAddress);
            instructions.push(Instruction::JumpIf);
        }
        Operator::Drop0 => {
            instructions.push(Instruction::Drop0);
        }
        Operator::Yield => {
            instructions.push(Instruction::Trigger {
                effect: Effect::Yield,
            });
        }
    }
}

fn translate_reference(name: &str, instructions: &mut Instructions) {
    let name = name.to_string();
    instructions.push(Instruction::Reference { name });
}
