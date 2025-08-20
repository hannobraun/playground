use std::path::{Path, PathBuf};

use clap::Parser;
use walkdir::WalkDir;

use crate::compiler::{
    input_code::read_input_code,
    tokens::{ProcessCharOutcome, Token, Tokenizer},
};

mod compiler;
#[cfg(test)]
mod runtime;
#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(program) = args.program {
        run(program, args.interactive)?;
    } else {
        for entry in WalkDir::new("examples") {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            let program = entry.path();
            run(program, args.interactive)?;

            println!("{program}", program = program.display());
        }
    }

    Ok(())
}

#[derive(clap::Parser)]
pub struct Args {
    pub program: Option<PathBuf>,

    #[arg(short, long)]
    pub interactive: bool,
}

fn run(program: impl AsRef<Path>, interactive: bool) -> anyhow::Result<()> {
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
                if interactive {
                    println!("Char: {ch}");
                }
            }
        }

        if interactive {
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
    }

    Ok(())
}

#[test]
fn single_number() -> anyhow::Result<()> {
    use crate::tests::compile;

    let stack = compile("examples/single-number.mbl")?;
    assert!(stack.is_empty());
    Ok(())
}
