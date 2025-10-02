mod parse;
mod tokenize;
mod translate;

use crate::{
    compiler::{
        parse::parse_operator,
        tokenize::{Token, tokenize},
        translate::{translate_label, translate_operator, translate_reference},
    },
    instructions::{Instructions, Labels},
};

pub fn compile(input: &str) -> (Instructions, Labels) {
    let tokens = tokenize(input);

    let mut instructions = Instructions::new();
    let mut labels = Labels::new();

    for token in tokens {
        match token {
            Token::Label { name } => {
                translate_label(name, &mut instructions, &mut labels);
            }
            Token::Operator { name } => {
                let operator = parse_operator(name);
                translate_operator(operator, &mut instructions);
            }
            Token::Reference { name } => {
                translate_reference(name, &mut instructions);
            }
        };
    }

    (instructions, labels)
}
