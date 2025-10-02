mod parse;
mod tokenize;
mod translate;

use crate::{
    compiler::{
        parse::{Expression, parse_script},
        tokenize::tokenize,
        translate::{translate_label, translate_operator, translate_reference},
    },
    instructions::{Instructions, Labels},
};

pub fn compile(input: &str) -> (Instructions, Labels) {
    let tokens = tokenize(input);
    let script = parse_script(tokens);

    let mut instructions = Instructions::new();
    let mut labels = Labels::new();

    let all_functions = [script.root]
        .map(|function| (None, function))
        .into_iter()
        .chain(
            script
                .functions
                .into_iter()
                .map(|(name, function)| (Some(name), function)),
        );

    for (name, function) in all_functions {
        if let Some(name) = name {
            translate_label(name, &mut instructions, &mut labels);
        }

        for expression in function.body {
            match expression {
                Expression::Operator { operator } => {
                    translate_operator(operator, &mut instructions);
                }
                Expression::Reference { name } => {
                    translate_reference(name, &mut instructions);
                }
            }
        }
    }

    (instructions, labels)
}
