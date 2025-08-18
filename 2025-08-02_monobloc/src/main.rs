use crate::compiler::{
    input_code::read_input_code,
    tokens::{ProcessCharOutcome, Tokenizer},
};
#[cfg(test)]
use crate::tests::compile;

mod compiler;
#[cfg(test)]
mod runtime;
#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let path = "examples/single-number.mbl";

    // We wouldn't need to create the buffer here, if `String::into_chars` were
    // stable:
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_chars
    let mut input_code = String::new();
    let mut input_code = read_input_code(path, &mut input_code)?;

    let mut tokenizer = Tokenizer::new();

    loop {
        match tokenizer.process_char(&mut input_code) {
            ProcessCharOutcome::NoMoreChars => {
                break;
            }
            ProcessCharOutcome::TokenIsReady { token } => {
                println!("Token: {token}");
            }
            ProcessCharOutcome::TokenNotReady { ch } => {
                println!("Char: {ch}");
            }
        }
    }

    Ok(())
}

#[test]
fn single_number() -> anyhow::Result<()> {
    let stack = compile("examples/single-number.mbl")?;
    assert_eq!(stack, vec![1]);
    Ok(())
}
