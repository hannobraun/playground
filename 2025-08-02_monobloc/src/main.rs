use std::path::PathBuf;

use clap::Parser;

use crate::compiler::{
    input_code::read_input_code,
    tokens::{ProcessCharOutcome, Token, Tokenizer},
};
#[cfg(test)]
use crate::tests::compile;

mod compiler;
#[cfg(test)]
mod runtime;
#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let program = args
        .program
        .unwrap_or_else(|| PathBuf::from("examples/single-number.mbl"));

    // We wouldn't need to create the buffer here, if `String::into_chars` were
    // stable:
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_chars
    let mut input_code = String::new();
    let mut input_code = read_input_code(program, &mut input_code)?;

    let mut tokenizer = Tokenizer::new();
    let mut tokens = Vec::new();

    loop {
        match tokenizer.process_char(&mut input_code) {
            ProcessCharOutcome::NoMoreChars => {
                break;
            }
            ProcessCharOutcome::TokenIsReady { token } => {
                tokens.push(token);
            }
            ProcessCharOutcome::TokenNotReady { ch } => {
                println!("Char: {ch}");
            }
        }

        let mut prev_token: Option<&Token> = None;

        for token in &tokens {
            match (prev_token, token) {
                (Some(Token::Comment { .. }), Token::Comment { .. }) => {
                    // Already printed a newline at the end of the previous
                    // comment.
                }
                (Some(_), Token::Comment { .. }) => {
                    // Start comment on a new line.
                    println!();
                }
                (Some(Token::Comment { .. }) | None, _) => {
                    // We are on a fresh line. Nothing to prepare.
                }
                (Some(_), _) => {
                    // Add other tokens to the same line.
                    print!(" ");
                }
            }

            match token {
                Token::Comment { text } => {
                    println!("#{text}");
                }
                Token::Identifier { name } => {
                    println!("{name}");
                }
                Token::Number { value } => {
                    println!("{value}");
                }
            }

            prev_token = Some(token);
        }
        println!();
    }

    Ok(())
}

#[derive(clap::Parser)]
pub struct Args {
    pub program: Option<PathBuf>,

    #[arg(short, long)]
    pub interactive: bool,
}

#[test]
fn single_number() -> anyhow::Result<()> {
    let stack = compile("examples/single-number.mbl")?;
    assert!(stack.is_empty());
    Ok(())
}
