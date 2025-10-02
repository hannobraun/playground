mod parse;
mod tokenize;
mod translate;

use crate::{
    compiler::{
        parse::parse_script, tokenize::tokenize, translate::translate_script,
    },
    instructions::{Instructions, Labels},
};

pub fn compile(input: &str) -> (Instructions, Labels) {
    let tokens = tokenize(input);
    let script = parse_script(tokens);
    translate_script(script)
}
