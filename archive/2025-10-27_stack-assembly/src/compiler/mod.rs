mod parse;
mod tokenize;
mod translate;

use crate::{
    compiler::{
        parse::parse_script, tokenize::tokenize, translate::translate_function,
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
        translate_function(name, function, &mut instructions, &mut labels);
    }

    (instructions, labels)
}
