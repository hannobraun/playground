use std::{fs::File, io::Write, path::Path};

use anyhow::Context;

use crate::{
    compiler::{
        input_code::read_input_code,
        ir::compile_tokens,
        tokens::{ProcessCharOutcome, Token, Tokenizer},
        wasm,
    },
    runtime,
};

pub fn compile(
    path: impl AsRef<Path>,
    interactive: bool,
) -> anyhow::Result<Vec<i32>> {
    // We wouldn't need to create the buffer here, if `String::into_chars` were
    // stable:
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_chars
    let mut input_code = String::new();
    let mut input_code = read_input_code(path, &mut input_code)?;

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

    let root = compile_tokens(tokens);
    let wasm_code = wasm::compile_module(&root);
    let stack = match runtime::evaluate_root(&wasm_code, &root) {
        Ok(stack) => stack,
        Err(err) => {
            let output = "error.wasm";
            File::create(output)?.write_all(&wasm_code)?;
            return Err(err).with_context(|| {
                format!("Error evaluating root; wrote module to `{output}`")
            });
        }
    };

    Ok(stack)
}
