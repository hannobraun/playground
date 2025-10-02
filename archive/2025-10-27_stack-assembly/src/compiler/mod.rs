mod parse;
mod translate;

use crate::{
    compiler::{
        parse::{parse_label, parse_operator, parse_reference},
        translate::{translate_label, translate_operator, translate_reference},
    },
    instructions::{Instructions, Labels},
};

pub fn compile(input: &str) -> (Instructions, Labels) {
    let mut instructions = Instructions::new();
    let mut labels = Labels::new();

    for token in input.split_whitespace() {
        if let Some(name) = parse_reference(token) {
            translate_reference(name, &mut instructions);
        } else if let Some(name) = parse_label(token) {
            translate_label(name, &mut instructions, &mut labels);
        } else {
            let operator = parse_operator(token);
            translate_operator(operator, &mut instructions);
        }
    }

    (instructions, labels)
}
