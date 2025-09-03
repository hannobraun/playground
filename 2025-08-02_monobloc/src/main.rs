use std::{fs::File, io::Write, path::Path};

use anyhow::Context;
use walkdir::WalkDir;

use crate::{
    args::Args,
    compiler::{
        inferrer::Inferrer,
        input_code::read_input_code,
        ir,
        nodes::{Node, NodeKind, Parser},
        resolver::Resolver,
        tokens::{IntegerFormat, Tokenizer},
        wasm,
    },
};

mod args;
mod compiler;
mod runtime;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(program) = args.program {
        compile(program, args.interactive)?;
    } else {
        for entry in WalkDir::new("examples") {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            let program = entry.path();
            compile(program, args.interactive).with_context(|| {
                format!("Compiling {path}", path = entry.path().display())
            })?;

            println!("OK {program}", program = program.display());
        }
    }

    Ok(())
}

pub fn compile(
    program: impl AsRef<Path>,
    interactive: bool,
) -> anyhow::Result<Vec<i32>> {
    // We wouldn't need to create the buffer here, if `String::into_chars` were
    // stable:
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_chars
    let mut input_code = String::new();
    let mut input_code = read_input_code(program, &mut input_code)?;

    let mut tokenizer = Tokenizer::new();
    let mut parser = Parser::new();
    let mut resolver = Resolver::new();
    let mut inferrer = Inferrer::new();

    let mut syntax = Vec::new();

    loop {
        let Some(ch) = input_code.next() else {
            break;
        };

        match tokenizer.process_char(ch) {
            Some(token) => {
                if let Some(node) = parser.process_token(token) {
                    resolver.process_node(&node);
                    inferrer.process_node(&node, &resolver);
                    syntax.push(node);
                }
            }
            None => {
                if interactive {
                    println!("Char: {ch}");
                }
            }
        }

        if interactive {
            let mut prev_node: Option<&NodeKind> = None;

            for node in &syntax {
                print_node(prev_node, node);
                prev_node = Some(&node.kind);
            }
            println!();
        }
    }

    let root = ir::generate(syntax, resolver, inferrer);
    let wasm_code = wasm::compile_module(&root);
    let stack = match runtime::evaluate_root(&wasm_code, &root) {
        Ok(stack) => stack,
        Err(err) => {
            let output = "error.wasm";
            File::create(output)?.write_all(&wasm_code)?;
            return Err(err).with_context(|| {
                format!("Failed to evaluate root; wrote module to `{output}`")
            });
        }
    };

    Ok(stack)
}

fn print_node(prev_node: Option<&NodeKind>, node: &Node) {
    match (prev_node, &node.kind) {
        (Some(NodeKind::Comment { .. }), NodeKind::Comment { .. }) => {
            // Already printed a newline at the end of the previous
            // comment.
        }
        (Some(_), NodeKind::Comment { .. }) => {
            // Start comment on a new line.
            println!();
        }
        (Some(NodeKind::Comment { .. }) | None, _) => {
            // We are on a fresh line. Nothing to prepare.
        }
        (Some(_), _) => {
            // Add other tokens to the same line.
            print!(" ");
        }
    }

    match &node.kind {
        NodeKind::Binding { names } => {
            print!("=> ");

            for name in names {
                print!("{name} ");
            }

            println!(".");
        }
        NodeKind::Block { nodes: _ } => {
            println!("{{ ... }}");
        }
        NodeKind::Comment { text } => {
            println!("#{text}");
        }
        NodeKind::Identifier { name } => {
            println!("{name}");
        }
        NodeKind::Integer {
            value,
            format: IntegerFormat::Hex,
        } => {
            println!("{value:x}");
        }
        NodeKind::Integer {
            value,
            format: IntegerFormat::Signed,
        } => {
            let value = i32::from_le_bytes(value.to_le_bytes());
            println!("{value}");
        }
        NodeKind::Integer {
            value,
            format: IntegerFormat::Unsigned,
        } => {
            println!("{value}");
        }
    }
}
