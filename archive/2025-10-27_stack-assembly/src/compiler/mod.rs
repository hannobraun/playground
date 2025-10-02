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
